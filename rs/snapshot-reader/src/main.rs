use std::path::Path;
use std::str::FromStr;

use dominion_protector::db::full_object::save_object;
use dominion_protector::db::Db;
use sui_core::authority::authority_store_tables::AuthorityPerpetualTables;
use sui_core::authority::authority_store_tables::LiveObject;
use sui_types::base_types::ObjectID;
use sui_types::object::Data;
use tokio::main;
use dominion_protector::db::object::Object;

#[main]
async fn main() -> anyhow::Result<()> {
    let perpetual_tables =
        AuthorityPerpetualTables::open(&Path::new("/mnt/f/sui/staging/store"), None);

    let db = Db::new().await?;

    let last_id = Object::last_id(&db.pool).await?;

    let mut i = 0;

    for obj in perpetual_tables.range_iter_live_object_set(last_id, None, false) {
        if let LiveObject::Normal(obj) = obj {
            if Some(obj.id()) == last_id {
                continue;
            }
            if let Data::Package(_) = &obj.data {
                println!("{}) Package: {}", i, obj.id());
                i += 1;
                let mut tx = db.pool.begin().await?;
                save_object(&mut *tx, "mainnet", &obj.data).await?;
                tx.commit().await?;
                println!("Saved package: {}", obj.id());
            }
        }
    }
    Ok(())
}
