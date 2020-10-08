use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::errors::MyError;
use crate::models::Position;

pub async fn get_pos(client: &Client) -> Result<Vec<Position>, MyError> {
    let statement = client
        .prepare("SELECT * FROM Position")
        .await
        .unwrap();
    let lists = client
        .query(&statement, &[])
        .await
        .expect("Error dans la requete")
        .iter()
        .map(|row| Position::from_row_ref(row).unwrap())
        .collect::<Vec<Position>>();
    Ok(lists)
}