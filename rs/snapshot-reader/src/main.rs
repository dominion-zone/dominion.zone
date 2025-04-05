use std::path::Path;

use sui_core::authority::authority_store_tables::AuthorityPerpetualTables;
use tokio::main;

#[main]
async fn main() -> anyhow::Result<()> {
    let perpetual_tables =
        AuthorityPerpetualTables::open(&Path::new("/mnt/f/sui/staging/store"), None);
    for obj in perpetual_tables.iter_live_object_set(false) {
        println!("Object: {:?}", obj);
    }
    Ok(())
}
