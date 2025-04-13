use actix_web::{web, App, HttpServer};
use crate::adapters::http::calculator_http_adapter;
use crate::app::calculator_app::CalculatorApp;
use crate::app::CalculatorApp;
use crate::domain::SimpleCalculator;

pub fn run_server() -> std::io::Result<()> {
    let app = calculator_http_adapter::new(CalculatorApp);
    let adapter = CalculatorHttpAdapter::new(app);

    HttpServer::new(move || {
        App::new()
            .route("/add", web::post().to(adapter.add))
            .route("/subtract", web::post().to(adapter.subtract))
            .route("/multiply", web::post().to(adapter.multiply))
            .route("/divide", web::post().to(adapter.divide))
    })
    .bind("127.0.0.1:8080")?
    .run()
}