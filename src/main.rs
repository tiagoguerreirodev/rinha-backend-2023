use std::net::SocketAddr;
use axum::response::{IntoResponse};
use axum::{Json, Router};
use axum::http::Response;
use axum::routing::{get, post};
use mongodb::{Client, Collection, Database, IndexModel};
use mongodb::bson::{DateTime, doc};
use mongodb::options::{IndexOptions, InsertOneOptions};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

    let routes = Router::new()
        .route("/pessoas", post(handler_pessoas_post))
        ;

    let address = SocketAddr::from(([0,0,0,0], 80));

    axum::Server::bind(&address)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn handler_pessoas_post(Json(body):Json<Pessoa>) {

    let database = Client::with_uri_str("mongodb://root:rinha123@localhost:27017")
        .await
        .unwrap();

    let collection: Collection<Pessoa> = database.database("rinha").collection("Pessoas");

    collection.insert_one(&body, InsertOneOptions::default()).await.expect("Could not save user");
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pessoa {
    apelido:String,
    nome:String,
    nascimento:String,
    stack: Vec<String>
}
