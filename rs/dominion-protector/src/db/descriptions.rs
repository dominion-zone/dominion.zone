use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use postgres_types::{FromSql, ToSql};
use sui_sdk::types::Identifier;
use tokio_postgres::{Client, Row, Transaction};

pub async fn create_description_tables_if_needed(client: &Client) -> Result<()> {
    /*
        CREATE TYPE security_level AS ENUM (
        'Critical Risk',
        'High Risk',
        'Medium Risk',
        'Low Risk',
        'Best Practices Compliant',
        'Unknown / Unassessed'
    );
    CREATE TYPE entity_kind AS ENUM ('parameter', 'created');
         */
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS module_descriptions (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            module          TEXT NOT NULL,
            description     TEXT NOT NULL,
            security_level  security_level NOT NULL DEFAULT 'Unknown / Unassessed',
            warnings        TEXT[] NOT NULL DEFAULT '{}',
            PRIMARY KEY(package_id, network, module),
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network),
            FOREIGN KEY(package_id, network, module) REFERENCES package_modules(package_id, network, name)
        );
    ").await?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS struct_descriptions (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            module          TEXT NOT NULL,
            struct          TEXT NOT NULL,
            description     TEXT NOT NULL,
            address_owned   TEXT,
            object_owned    TEXT,
            wrapped           TEXT,
            shared          TEXT,
            immutable       TEXT,
            warnings        TEXT[] NOT NULL DEFAULT '{}',
            PRIMARY KEY(package_id, network, module, struct),
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network),
            FOREIGN KEY(package_id, network, module) REFERENCES package_modules(package_id, network, name)
        );
    ").await?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS function_descriptions (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            module          TEXT NOT NULL,
            function        TEXT NOT NULL,
            description     TEXT NOT NULL,
            security_level  security_level NOT NULL DEFAULT 'Unknown / Unassessed',
            warnings        TEXT[] NOT NULL DEFAULT '{}',
            PRIMARY KEY(package_id, network, module, function),
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network),
            FOREIGN KEY(package_id, network, module) REFERENCES package_modules(package_id, network, name)
        );
    ").await?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS function_entity_descriptions (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            module          TEXT NOT NULL,
            function        TEXT NOT NULL,
            kind            entity_kind NOT NULL,
            index           INT,
            name            TEXT NOT NULL,
            proper_name     TEXT,
            description     TEXT NOT NULL,
            type            TEXT NOT NULL,
            returned        TEXT,
            transferred     TEXT,
            shared          TEXT,
            frozen          TEXT,
            wrapped         TEXT,
            modified        TEXT,
            dropped         TEXT,
            warnings        TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network),
            FOREIGN KEY(package_id, network, module) REFERENCES package_modules(package_id, network, name),
            FOREIGN KEY(package_id, network, module, function) REFERENCES function_descriptions(package_id, network, module, function)
        );
    ").await?;
    Ok(())
}

#[derive(Debug, Clone, Copy, ToSql, FromSql)]
#[postgres(name = "security_level")]
pub enum SecurityLevel {
    #[postgres(name = "Critical Risk")]
    CriticalRisk,
    #[postgres(name = "High Risk")]
    HighRisk,
    #[postgres(name = "Medium Risk")]
    MediumRisk,
    #[postgres(name = "Low Risk")]
    LowRisk,
    #[postgres(name = "Best Practices Compliant")]
    BestPracticesCompliant,
    #[postgres(name = "Unknown / Unassessed")]
    UnknownUnassessed,
}

#[derive(Debug, Clone, Copy, ToSql, FromSql)]
#[postgres(name = "entity_kind")]
enum EntityKind {
    #[postgres(name = "parameter")]
    Parameter,
    #[postgres(name = "created")]
    Created,
}

#[derive(Debug, Clone)]
pub struct ModuleDescription {
    pub id: ModuleId,
    pub network: String,
    pub description: String,
    pub security_level: SecurityLevel,
    pub warnings: Vec<String>,
}

impl TryFrom<&Row> for ModuleDescription {
    type Error = anyhow::Error;

    fn try_from(value: &Row) -> Result<Self> {
        let columns = value.columns();
        let package_id: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "package_id")
                .context("package_id column not found")?,
        );
        let network: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "network")
                .context("network column not found")?,
        );
        let module: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "module")
                .context("module column not found")?,
        );
        let description: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "description")
                .context("description column not found")?,
        );
        let security_level: SecurityLevel = value.get(
            columns
                .iter()
                .position(|c| c.name() == "security_level")
                .context("security_level column not found")?,
        );
        let warnings: Vec<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "warnings")
                .context("warnings column not found")?,
        );
        Ok(Self {
            id: ModuleId::new(
                AccountAddress::from_hex_literal(&package_id)?,
                Identifier::from_str(&module)?,
            ),
            network,
            description,
            security_level,
            warnings,
        })
    }
}

impl ModuleDescription {
    const COLUMNS: &'static [&'static str] = &[
        "package_id",
        "network",
        "module",
        "description",
        "security_level",
        "warnings",
    ];

    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Option<ModuleDescription>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
    FROM module_descriptions
    WHERE package_id = $1 AND network = $2 AND module = $3",
                    ModuleDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(None);
        }
        return Ok(Some(ModuleDescription::try_from(&rows[0])?));
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let rows = tx.execute(
            &format!(
                "INSERT INTO module_descriptions ({})
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT (package_id, network, module) DO UPDATE
    SET description = $4, security_level = $5, warnings = $6",
                ModuleDescription::COLUMNS.join(", ")
            ),
            &[
                &self.id.address().to_hex_literal(),
                &self.network,
                &self.id.name().as_str(),
                &self.description,
                &self.security_level,
                &self.warnings,
            ],
        ).await?;
        Ok(rows > 0)
    }
}

#[derive(Debug, Clone)]
pub struct FullModuleDescription {
    pub module: ModuleDescription,
    pub structs: Vec<StructDescription>,
    pub functions: Vec<FullFunctionDescription>,
}

impl FullModuleDescription {
    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Option<Self>> {
        let module =
            ModuleDescription::read_from_db(module_id.clone(), network.clone(), db).await?;
        if let Some(module) = module {
            let structs =
                StructDescription::read_all_from_db(module_id.clone(), network.clone(), db).await?;
            let functions =
                FullFunctionDescription::read_all_from_db(module_id.clone(), network.clone(), db)
                    .await?;
            return Ok(Some(Self {
                module,
                structs,
                functions,
            }));
        } else {
            Ok(None)
        }
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let module_saved = self.module.save_to_db(tx).await?;
        tx.execute("DELETE FROM struct_descriptions
        WHERE package_id = $1 AND network = $2 AND module = $3" , 
        &[
            &self.module.id.address().to_hex_literal(),
            &self.module.network,
            &self.module.id.name().as_str(),
        ]).await?;
        tx.execute("DELETE FROM function_descriptions
        WHERE package_id = $1 AND network = $2 AND module = $3 CASCADE" , 
        &[
            &self.module.id.address().to_hex_literal(),
            &self.module.network,
            &self.module.id.name().as_str(),
        ]).await?;
        for struct_ in &self.structs {
            struct_.save_to_db(tx).await?;
        }
        for function in &self.functions {
            function.save_to_db(tx).await?;
        }
        Ok(module_saved)
    }
}

#[derive(Debug, Clone)]
pub struct StructDescription {
    pub module: ModuleId,
    pub network: String,
    pub struct_name: String,
    pub description: String,
    pub address_owned: Option<String>,
    pub object_owned: Option<String>,
    pub wrapped: Option<String>,
    pub shared: Option<String>,
    pub immutable: Option<String>,
    pub warnings: Vec<String>,
}

impl StructDescription {
    const COLUMNS: &'static [&'static str] = &[
        "package_id",
        "network",
        "module",
        "struct",
        "description",
        "address_owned",
        "object_owned",
        "wrapped",
        "shared",
        "immutable",
        "warnings",
    ];

    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        struct_name: String,
        db: &mut Client,
    ) -> Result<Option<StructDescription>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
    FROM struct_descriptions
    WHERE package_id = $1 AND network = $2 AND module = $3 AND struct = $4",
                    StructDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                    &struct_name,
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(None);
        }
        return Ok(Some(StructDescription::try_from(&rows[0])?));
    }

    pub async fn read_all_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Vec<StructDescription>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
    FROM struct_descriptions
    WHERE package_id = $1 AND network = $2 AND module = $3",
                    StructDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                ],
            )
            .await?;
        return rows
            .into_iter()
            .map(|row| StructDescription::try_from(&row))
            .collect();
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let rows = tx.execute(
            &format!(
                "INSERT INTO struct_descriptions ({})
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    ON CONFLICT (package_id, network, module, struct) DO UPDATE
    SET description = $4, address_owned = $5, object_owned = $6, wrapped = $7, shared = $8, immutable = $9, warnings = $10",
                StructDescription::COLUMNS.join(", ")
            ),
            &[
                &self.module.address().to_hex_literal(),
                &self.network,
                &self.module.name().as_str(),
                &self.struct_name,
                &self.description,
                &self.address_owned,
                &self.object_owned,
                &self.wrapped,
                &self.shared,
                &self.immutable,
                &self.warnings,
            ],
        ).await?;
        Ok(rows > 0)
    }
}

impl TryFrom<&Row> for StructDescription {
    type Error = anyhow::Error;

    fn try_from(value: &Row) -> Result<Self> {
        let columns = value.columns();
        let package_id: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "package_id")
                .context("package_id column not found")?,
        );
        let network: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "network")
                .context("network column not found")?,
        );
        let module: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "module")
                .context("module column not found")?,
        );
        let struct_: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "struct")
                .context("struct column not found")?,
        );
        let description: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "description")
                .context("description column not found")?,
        );
        let address_owned: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "address_owned")
                .context("address_owned column not found")?,
        );
        let object_owned: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "object_owned")
                .context("object_owned column not found")?,
        );
        let wrapped: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "wrapped")
                .context("wrapped column not found")?,
        );
        let shared: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "shared")
                .context("shared column not found")?,
        );
        let immutable: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "immutable")
                .context("immutable column not found")?,
        );
        let warnings: Vec<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "warnings")
                .context("warnings column not found")?,
        );
        Ok(Self {
            module: ModuleId::new(
                AccountAddress::from_hex_literal(&package_id)?,
                Identifier::from_str(&module)?,
            ),
            network,
            struct_name: struct_,
            description,
            address_owned,
            object_owned,
            wrapped,
            shared,
            immutable,
            warnings,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDescription {
    pub module: ModuleId,
    pub network: String,
    pub function_name: String,
    pub description: String,
    pub security_level: SecurityLevel,
    pub warnings: Vec<String>,
}

impl FunctionDescription {
    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        function_name: String,
        db: &mut Client,
    ) -> Result<Option<Self>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
    FROM function_descriptions
    WHERE package_id = $1 AND network = $2 AND module = $3 AND function = $4",
                    FunctionDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                    &function_name,
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(None);
        }
        return Ok(Some(FunctionDescription::try_from(&rows[0])?));
    }

    pub async fn read_all_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Vec<FunctionDescription>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
    FROM function_descriptions
    WHERE package_id = $1 AND network = $2 AND module = $3",
                    FunctionDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                ],
            )
            .await?;
        return rows
            .into_iter()
            .map(|row| FunctionDescription::try_from(&row))
            .collect();
    }

    async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let rows = tx.execute(
            &format!(
                "INSERT INTO function_descriptions ({})
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT (package_id, network, module, function) DO UPDATE
    SET description = $4, security_level = $5, warnings = $6",
                FunctionDescription::COLUMNS.join(", ")
            ),
            &[
                &self.module.address().to_hex_literal(),
                &self.network,
                &self.module.name().as_str(),
                &self.function_name,
                &self.description,
                &self.security_level,
                &self.warnings,
            ],
        ).await?;
        Ok(rows > 0)
    }
}

impl TryFrom<&Row> for FunctionDescription {
    type Error = anyhow::Error;

    fn try_from(value: &Row) -> Result<Self> {
        let columns = value.columns();
        let package_id: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "package_id")
                .context("package_id column not found")?,
        );
        let network: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "network")
                .context("network column not found")?,
        );
        let module: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "module")
                .context("module column not found")?,
        );
        let function: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "function")
                .context("function column not found")?,
        );
        let description: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "description")
                .context("description column not found")?,
        );
        let security_level: SecurityLevel = value.get(
            columns
                .iter()
                .position(|c| c.name() == "security_level")
                .context("security_level column not found")?,
        );
        let warnings: Vec<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "warnings")
                .context("warnings column not found")?,
        );

        Ok(Self {
            module: ModuleId::new(
                AccountAddress::from_hex_literal(&package_id)?,
                Identifier::from_str(&module)?,
            ),
            network,
            function_name: function,
            description,
            security_level,
            warnings,
        })
    }
}

impl FunctionDescription {
    const COLUMNS: &'static [&'static str] = &[
        "package_id",
        "network",
        "module",
        "function",
        "description",
        "security_level",
        "warnings",
    ];
}

#[derive(Debug, Clone)]
pub struct FullFunctionDescription {
    pub function: FunctionDescription,
    pub entities: Vec<FunctionEntityDescription>,
}

impl FullFunctionDescription {
    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        function_name: String,
        db: &mut Client,
    ) -> Result<Option<Self>> {
        let function = FunctionDescription::read_from_db(
            module_id.clone(),
            network.clone(),
            function_name.clone(),
            db,
        )
        .await?
        .ok_or_else(|| anyhow::anyhow!("Function not found"))?;
        let entities =
            FunctionEntityDescription::read_from_db(module_id, network, function_name, db).await?;
        Ok(Some(Self { function, entities }))
    }

    pub async fn read_all_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Vec<Self>> {
        let functions =
            FunctionDescription::read_all_from_db(module_id.clone(), network.clone(), db).await?;
        let mut result = vec![];
        for function in functions {
            let entities = FunctionEntityDescription::read_from_db(
                module_id.clone(),
                network.clone(),
                function.function_name.clone(),
                db,
            )
            .await?;
            result.push(Self { function, entities });
        }
        Ok(result)
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let function_saved = self.function.save_to_db(tx).await?;
        tx.execute("DELETE FROM function_entity_descriptions
        WHERE package_id = $1 AND network = $2 AND module = $3 AND function = $4", 
        &[
            &self.function.module.address().to_hex_literal(),
            &self.function.network,
            &self.function.module.name().as_str(),
            &self.function.function_name,
        ]).await?;
        for entity in &self.entities {
            entity.save_to_db(tx).await?;
        }
        Ok(function_saved)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionEntityDescription {
    pub module: ModuleId,
    pub network: String,
    pub function: String,
    pub description: String,
    pub kind: EntityKind,
    pub index: Option<i32>,
    pub name: String,
    pub proper_name: Option<String>,
    pub type_: String,
    pub returned: Option<String>,
    pub transferred: Option<String>,
    pub shared: Option<String>,
    pub frozen: Option<String>,
    pub wrapped: Option<String>,
    pub modified: Option<String>,
    pub dropped: Option<String>,
    pub warnings: Vec<String>,
}

impl TryFrom<&Row> for FunctionEntityDescription {
    type Error = anyhow::Error;

    fn try_from(value: &Row) -> Result<Self> {
        let columns = value.columns();
        let package_id: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "package_id")
                .context("package_id column not found")?,
        );
        let network: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "network")
                .context("network column not found")?,
        );
        let module: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "module")
                .context("module column not found")?,
        );
        let function: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "function")
                .context("function column not found")?,
        );
        let kind: EntityKind = value.get(
            columns
                .iter()
                .position(|c| c.name() == "kind")
                .context("kind column not found")?,
        );
        let index: Option<i32> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "index")
                .context("index column not found")?,
        );

        let description: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "description")
                .context("description column not found")?,
        );

        let name = value.get(
            columns
                .iter()
                .position(|c| c.name() == "name")
                .context("name column not found")?,
        );

        let proper_name = value.get(
            columns
                .iter()
                .position(|c| c.name() == "proper_name")
                .context("proper_name column not found")?,
        );

        let type_ = value.get(
            columns
                .iter()
                .position(|c| c.name() == "type")
                .context("type column not found")?,
        );

        let returned = value.get(
            columns
                .iter()
                .position(|c| c.name() == "returned")
                .context("returned column not found")?,
        );

        let transferred = value.get(
            columns
                .iter()
                .position(|c| c.name() == "transferred")
                .context("transferred column not found")?,
        );

        let shared = value.get(
            columns
                .iter()
                .position(|c| c.name() == "shared")
                .context("shared column not found")?,
        );

        let frozen = value.get(
            columns
                .iter()
                .position(|c| c.name() == "frozen")
                .context("frozen column not found")?,
        );

        let wrapped = value.get(
            columns
                .iter()
                .position(|c| c.name() == "wrapped")
                .context("wrapped column not found")?,
        );

        let modified = value.get(
            columns
                .iter()
                .position(|c| c.name() == "modified")
                .context("modified column not found")?,
        );

        let dropped = value.get(
            columns
                .iter()
                .position(|c| c.name() == "dropped")
                .context("dropped column not found")?,
        );

        let warnings = value.get(
            columns
                .iter()
                .position(|c| c.name() == "warnings")
                .context("warnings column not found")?,
        );

        Ok(Self {
            module: ModuleId::new(
                AccountAddress::from_hex_literal(&package_id)?,
                Identifier::from_str(&module)?,
            ),
            network,
            function,
            kind,
            index,
            name,
            proper_name,
            description,
            type_,
            returned,
            transferred,
            shared,
            frozen,
            wrapped,
            modified,
            dropped,
            warnings,
        })
    }
}

impl FunctionEntityDescription {
    const COLUMNS: &'static [&'static str] = &[
        "package_id",
        "network",
        "module",
        "function",
        "kind",
        "index",
        "name",
        "proper_name",
        "description",
        "type",
        "returned",
        "transferred",
        "shared",
        "frozen",
        "wrapped",
        "modified",
        "dropped",
        "warnings",
    ];

    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        function_name: String,
        db: &mut Client,
    ) -> Result<Vec<Self>> {
        let rows = db
            .query(
                &format!("SELECT {}
        FROM function_entity_descriptions WHERE package_id = $1 AND network = $2 AND module = $3 AND function = $4",
                FunctionEntityDescription::COLUMNS.join(", ")),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                    &function_name,
                ],
            )
            .await?;

        return rows
            .into_iter()
            .map(|row| FunctionEntityDescription::try_from(&row))
            .collect();
    }

    async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let rows = tx.execute(
            &format!(
                "INSERT INTO function_entity_descriptions ({})
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)",
                FunctionEntityDescription::COLUMNS.join(", ")
            ),
            &[
                &self.module.address().to_hex_literal(),
                &self.network,
                &self.module.name().as_str(),
                &self.function,
                &self.kind,
                &self.index,
                &self.name,
                &self.proper_name,
                &self.description,
                &self.type_,
                &self.returned,
                &self.transferred,
                &self.shared,
                &self.frozen,
                &self.wrapped,
                &self.modified,
                &self.dropped,
                &self.warnings,
            ],
        ).await?;
        Ok(rows > 0)
    }
}
