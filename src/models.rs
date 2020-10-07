use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "position")] // singular 'user' is a keyword..
pub struct Position {
    pub id: i32,
    pub longitude: f32,
    pub latitude: f32,
}