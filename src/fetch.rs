use std::fs::File;
use std::io::prelude::*;

use anyhow::Result;
use log::debug;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

struct PgDatabase {
    host: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

impl PgDatabase {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

static CONFIG: Lazy<PgDatabase> = Lazy::new(|| PgDatabase {
    host: std::env::var("ITC_PG_HOST").unwrap(),
    port: std::env::var("ITC_PG_PORT").unwrap(),
    user: std::env::var("ITC_PG_USER").unwrap(),
    password: std::env::var("ITC_PG_PASSWORD").unwrap(),
    database: std::env::var("ITC_PG_DATABASE").unwrap(),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
struct Table {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub length: Option<i32>,
}

pub async fn fetch_db_info() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&CONFIG.url())
        .await?;

    let tables = sqlx::query_as::<_, Table>(
        "
with tables as (
  select
    tablename
  from
    pg_tables
  where
    schemaname not in ('pg_catalog', 'information_schema'))
select
  i.table_name,
  i.column_name,
  i.data_type,
  i.character_maximum_length as length
from
  information_schema.columns i
  inner join
    tables t
  on
    i.table_name = t.tablename
",
    )
    .fetch_all(&pool)
    .await?;

    let _ = dump_json(tables);
    Ok(())
}

fn dump_json(tables: Vec<Table>) -> Result<()> {
    let serialized = serde_json::to_string(&tables)?;
    let mut file = File::create("itc_db_info.json")?;
    file.write_all(serialized.as_bytes())?;

    debug!("itc_db_info.json is dumped");
    Ok(())
}
