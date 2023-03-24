use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, PostgresMapper, Deserialize, Serialize)]
#[pg_mapper(table = "records")]
pub struct Record {
    pub id: i32,
    pub label: String,
}
