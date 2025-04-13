// src/adapters/http/calculator_http_adapter.rs

use actix_web::{web, Responder, HttpResponse};
use serde_json::json;
use std::sync::Arc;
use crate::app::calculator_app::CalculatorApp;
use crate::domain::models::CalculationRequest;

pub struct CalculatorHttpAdapter {
    app: Arc<CalculatorApp>,
}

impl CalculatorHttpAdapter {
    pub fn new(app: Arc<CalculatorApp>) -> Self {
        CalculatorHttpAdapter { app }
    }

    async fn handle_calculation<F>(&self, request: CalculationRequest, operation: F) -> impl Responder
    where
        F: FnOnce(&Self, &CalculationRequest) -> Result<i32, String>,
    {
        let result = operation(self, &request);
        match result {
            Ok(res) => HttpResponse::Ok().json(json!({ "result": res })),
            Err(e) => HttpResponse::BadRequest().json(json!({ "error": e })),
        }
    }

    pub async fn add(&self, request: CalculationRequest) -> impl Responder {
        self.handle_calculation(request, |adapter, req| adapter.app.add(req.a, req.b)).await
    }

    pub async fn subtract(&self, request: CalculationRequest) -> impl Responder {
        self.handle_calculation(request, |adapter, req| adapter.app.subtract(req.a, req.b)).await
    }

    pub async fn multiply(&self, request: CalculationRequest) -> impl Responder {
        self.handle_calculation(request, |adapter, req| adapter.app.multiply(req.a, req.b)).await
    }

    pub async fn divide(&self, request: CalculationRequest) -> impl Responder {
        self.handle_calculation(request, |adapter, req| adapter.app.divide(req.a, req.b)).await
    }
}