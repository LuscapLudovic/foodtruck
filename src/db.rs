use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::errors::MyError;
use crate::models::Position;

/*pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let _stmt = include_str!("../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}*/

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