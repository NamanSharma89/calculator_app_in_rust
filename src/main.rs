// src/main.rs

mod app;
mod domain;
mod adapters;


use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::DefaultHeaders;
use adapters::http::calculator_http_adapter::CalculatorHttpAdapter;
use crate::app::calculator_app::CalculatorApp;
use crate::domain::models::CalculationRequest;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = Arc::new(CalculatorApp::new());
    let adapter = CalculatorHttpAdapter::new(app);
    let shared_adapter = web::Data::new(adapter);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_adapter.clone())
            .route("/add", web::post().to(handle_add))
            .route("/subtract", web::post().to(handle_subtract))
            .route("/multiply", web::post().to(handle_multiply))
            .route("/divide", web::post().to(handle_divide))
            .wrap(
                DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
                    .add(("Access-Control-Allow-Headers", "Content-Type"))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn handle_add(
    req: web::Json<CalculationRequest>,
    adapter: web::Data<CalculatorHttpAdapter>,
) -> impl actix_web::Responder {
    adapter.add(req.into_inner()).await
}

async fn handle_subtract(
    req: web::Json<CalculationRequest>,
    adapter: web::Data<CalculatorHttpAdapter>,
) -> impl actix_web::Responder {
    adapter.subtract(req.into_inner()).await
}

async fn handle_multiply(
    req: web::Json<CalculationRequest>,
    adapter: web::Data<CalculatorHttpAdapter>,
) -> impl actix_web::Responder {
    adapter.multiply(req.into_inner()).await
}

async fn handle_divide(
    req: web::Json<CalculationRequest>,
    adapter: web::Data<CalculatorHttpAdapter>,
) -> impl actix_web::Responder {
    adapter.divide(req.into_inner()).await
}