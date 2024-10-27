#![allow(warnings)]

use actix_web::{get, web::{self, Query}, App, HttpServer, Responder};
use actix::{Actor, Arbiter, System, Context};
use future::join_all;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use serde_json::*;
use futures::*;
use reqwest::*;

#[derive(Serialize, Deserialize)]
struct User {
    id: usize,
    name: String,
}

impl Actor for User {
    type Context = Context<Self>;
}

impl User {
    fn new() -> Self {
        User { id: 0, name: "None".to_string() }
    }
}

#[derive(Deserialize)]
struct QueryParams {
    id: usize,
    name: String,
}

#[get("/api/user")]
async fn user(query: Query<QueryParams>) -> impl Responder {
    to_string(&User {
        id: query.id,
        name: query.name.clone(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let urls = vec![
        "https://www.googel.com",
        "https://www.yandex.com",
        "https://www.vk.com",
        "https://www.ok.ru",
        "https://www.apple.com",
        "https://sobaka.ru",
        "https://www.rambler.ru",
        "https://cloudflare.com",
        "https://instagram.com",
        "https://facebook.ru",
        "https://meta.com"
    ];

    let fetches: Vec<_> = urls.iter().map(|&url| async move {
        reqwest::get(url).await
    }).collect();

    let results = join_all(fetches).await;

    for result in results {
        match result {
            Ok(data) => println!("OK"),
            Err(e) => println!("Error"),
        }
    }

    HttpServer::new(|| {
        App::new()
            .service(user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
