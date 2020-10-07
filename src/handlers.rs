use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

use crate::db;

pub async fn get_pos(db_pool: web::Data<Pool>) -> impl Responder {
	let client: Client = db_pool
		.get()
		.await
		.expect("Error de connexion à la base");
	let result = db::get_pos(&client).await;

	match result {
		Ok(lists) => HttpResponse::Ok().json(lists),
		Err(_) => HttpResponse::InternalServerError().into(),
	}
}