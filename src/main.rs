mod binance;
mod alice;

use actix_web::{App, HttpServer, HttpResponse, Responder, web, post};

use std::env::args;
use crate::binance::APIv3;
use crate::alice::{Alice, BackendRequest};
use reqwest::Request;
use std::borrow::Borrow;
use std::ops::Deref;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let token = args().nth(1).expect("Please specify a token");

    let api = APIv3::new(token);
    let alice = Alice::new(api);

    println!("listening on port {}", port);
    HttpServer::new(move || {
        App::new()
            .data(alice.clone())
            .route("/v1.0", web::get().to(health_check))
            .service(handle_alice_request)
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}

async fn health_check() -> impl Responder { HttpResponse::Ok() }

#[post("/alice/handler")]
async fn handle_alice_request(
    alice: web::Data<Box<Alice>>,
    request: web::Json<BackendRequest>
) -> impl Responder {
    let req = request.into_inner();
    let resp = alice.handle(req).await.unwrap();

    HttpResponse::Ok().json(resp)
}
