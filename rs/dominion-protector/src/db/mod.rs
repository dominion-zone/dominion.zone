use anyhow::Result;
use std::env;

// pub mod descriptions;
pub mod full_object;
pub mod function;
pub mod object;
pub mod package_linkage;
pub mod package_module;
pub mod structure;
pub mod sources;

pub struct Db {
    pub pool: sqlx::PgPool,
}

impl Db {
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")?;
        let pool = sqlx::PgPool::connect(&database_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}
