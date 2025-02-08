use std::str::FromStr;

use crate::{
    commands::{
        decompile::{decompile, get_or_decompile_module, DecompileParams},
        download::{download_object, get_or_download_object, DownloadObjectParams},
    },
    db::{
        build_db,
        descriptions::{self, create_description_tables_if_needed, ModuleDescription},
        sources::{read_source_from_db, ModuleSource},
    },
    sui_client::{build_client, SuiClientWithNetwork},
};
use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use openai_dive::v1::{
    api::Client as AIClient,
    resources::chat::{
        ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage,
        ChatMessageContent,
    },
};
use sui_sdk::{
    rpc_types::SuiRawData,
    types::{base_types::ObjectID, Identifier},
};
use tokio_postgres::Client;

#[derive(Args)]
pub struct DescribeCommand {
    #[command(subcommand)]
    command: DescribeType,
}

#[derive(Subcommand)]
enum DescribeType {
    Package { package_id: String },
    Module { full_name: String },
    Transaction { digest: String },
}

impl DescribeCommand {
    pub async fn run(self) -> Result<()> {
        let client = build_client().await?;
        let mut db = build_db().await?;
        let ai_api_key = std::env::var("AI_API_KEY").expect("AI_API_KEY is not set");
        create_description_tables_if_needed(&mut db).await?;
        println!("AI API key: {}", ai_api_key);

        let mut ai = AIClient::new(ai_api_key);
        ai.set_base_url("https://api.atoma.network/v1");
        match self.command {
            DescribeType::Package { package_id } => {
                println!("Describing package with ID: {}", package_id);
                let object_id = ObjectID::from_str(&package_id)?;
                let package = get_or_download_object(DownloadObjectParams {
                    object_id,
                    client: &client,
                    db: &mut db,
                })
                .await?;
                if let SuiRawData::Package(package) = package.bcs.unwrap() {
                    for (name, _) in &package.module_map {
                        let module_id =
                            ModuleId::new(object_id.into(), Identifier::new(name.clone())?);
                        let module =
                            read_source_from_db(module_id.clone(), client.network.clone(), &mut db)
                                .await?;
                        let module = if let Some(module) = module {
                            module
                        } else {
                            let modules = decompile(DecompileParams {
                                db: &mut db,
                                network: client.network.clone(),
                                package: &package,
                            })
                            .await?;
                            modules
                                .get(module_id.name())
                                .context("Module not found")?
                                .clone()
                        };
                        describe_module(DescribeModuleParams {
                            module,
                            db: &mut db,
                            client: &client,
                            ai: &ai,
                        })
                        .await?;
                    }
                } else {
                    bail!("Object is not a package");
                }
                Ok(())
            }
            DescribeType::Module { full_name } => {
                let module_id = ModuleId::from_str(&full_name)?;
                println!("Describing module: {}", &module_id);
                let module = get_or_decompile_module(module_id, &client, &mut db).await?;
                describe_module(DescribeModuleParams {
                    module,
                    db: &mut db,
                    client: &client,
                    ai: &ai,
                })
                .await?;
                Ok(())
            }
            DescribeType::Transaction { digest } => {
                println!("Describing transaction with digest: {}", digest);
                Ok(())
            }
        }
    }
}

const SYSTEM_TEXT: &str = "You are an expert in code analysis and smart contract auditing.  

📌 **Context**: You are working with the **source code of a module** from a smart contract on the SUI blockchain, **decompiled** from bytecode into the Move language.  
Additionally, you are provided with **descriptions of dependencies**, which were previously generated with your assistance.  

⚠️ **Important**: Since the code was **decompiled**, meaningful names are present **only for structures, their fields, and functions**.  
❌ **You must not rely solely on these names**—the contract code may be **low quality or intentionally obfuscated** to mislead the reader.  
✅ **Your task** is to analyze **code behavior**, its **structure**, **function calls**, **data usage**, and **interactions with dependencies**.  

---

## **🔍 How You Should Analyze the Code**
1️⃣ **You receive the module's source code and dependency descriptions**  
2️⃣ **You analyze how the functions operate**:
   - What parameters do they take?
   - What values do they return?
   - How do they interact with other parts of the code?  
3️⃣ **Be aware that dependency descriptions also contain information about how structures are owned and how functions manage them in terms of ownership.**  
4️⃣ **Use this information to understand ownership aspects in the context of the provided module.**  
5️⃣ **If the code is complex or obfuscated, try to reconstruct its logic.**  
6️⃣ **Answer questions about the code**, which will be asked in subsequent requests.  
7️⃣ **Strictly follow the expected response structure, as described in each request.**  

---

## **🔍 Response Structure**
🔹 **Your responses must strictly follow the structure described in the request.**  
🔹 If the request expects a **Yes/No** answer, respond **only with 'Yes' or 'No'**.  
🔹 If a structured response is required, follow the format exactly.  
🔹 If JSON output is expected, return a properly formatted JSON object.  

---

## **🔍 Requesting Additional Information**
🔹 **If there are unclear aspects of the code and you need more information about a function or structure, request it using this exact format**:  
✅ `request function <name>` → This will provide the **source code** and **dependency descriptions** for the specified function.  
✅ `request struct <name>` → This will provide the **source code** and **dependency descriptions** for the specified structure.  

📌 **Always use this format exactly when requesting missing data.**  
📌 **Do not request unnecessary information—only ask for what is required to complete the analysis.**  
📌 **If clarification is needed, structure your request concisely.**  

✅ **If there are unclear aspects of the code, you must explicitly request the missing details using this format.**";

pub struct DescribeModuleParams<'a> {
    pub module: ModuleSource,
    pub client: &'a SuiClientWithNetwork,
    pub db: &'a mut Client,
    pub ai: &'a AIClient,
}
async fn describe_module(
    DescribeModuleParams {
        module,
        client,
        db,
        ai,
    }: DescribeModuleParams<'_>,
) -> Result<ModuleDescription> {
    Box::pin(async move {
        let system_message = ChatMessage::Developer {
            content: ChatMessageContent::Text(SYSTEM_TEXT.to_string()),
            name: None,
        };
        let target_message = ChatMessage::User {
            content: ChatMessageContent::Text(format!(
                "Decomplied module for audit: ```move {}```",
                &module.source
            )),
            name: None,
        };
        let mut messages = vec![system_message];
        for dependency in &module.dependencies {
            if *dependency.address() == AccountAddress::ONE
                || *dependency.address() == AccountAddress::TWO
            {
                continue;
            }
            println!("Recursively describe {:?}", dependency);
            let description = get_or_describe_module(dependency.clone(), db, client, ai).await?;
            messages.push(ChatMessage::Assistant {
                content: Some(ChatMessageContent::Text(format!(
                    "Dependency {} description ```json {}```",
                    &description.id.to_canonical_display(true),
                    &description.description
                ))),
                reasoning_content: None,
                refusal: None,
                name: None,
                tool_calls: None,
            });
        }
        messages.push(target_message);
        let parameters = ChatCompletionParametersBuilder::default()
            .model("deepseek-ai/DeepSeek-R1")
            .messages(messages)
            .response_format(ChatCompletionResponseFormat::JsonObject)
            .build()?;

        let result = ai.chat().create(parameters).await?;
        if let ChatMessage::Assistant { content, .. } = &result.choices[0].message {
            let response: serde_json::Value =
                match content.as_ref().context("No content in response")? {
                    ChatMessageContent::Text(text) => {
                        println!("response {}", text);
                        serde_json::from_str(&text)?
                    }
                    ChatMessageContent::ContentPart(_) => todo!("ContentPart"),
                    ChatMessageContent::None => bail!("No content in response"),
                };
            db.execute(
                "INSERT INTO module_descriptions(
            package_id,
            network,
            module,
            description)
        VALUES ($1, $2, $3, $4)",
                &[
                    &module.id.address().to_hex_literal(),
                    &client.network,
                    &module.id.name().as_str(),
                    &response,
                ],
            )
            .await?;
            todo!()
        } else {
            bail!("Unexpected response from AI");
        }
    })
    .await
}

pub async fn get_or_describe_module(
    module_id: ModuleId,
    db: &mut Client,
    client: &SuiClientWithNetwork,
    ai: &AIClient,
) -> Result<ModuleDescription> {
    let module =
        ModuleDescription::read_from_db(module_id.clone(), client.network.clone(), db).await?;
    if let Some(module) = module {
        Ok(module)
    } else {
        let module = get_or_decompile_module(module_id, client, db).await?;
        describe_module(DescribeModuleParams {
            module,
            db,
            client,
            ai,
        })
        .await
    }
}
