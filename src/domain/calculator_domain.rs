use serde::{Deserialize, Serialize};
pub(crate) use crate::domain::models::CalculationRequest;

pub trait Calculator {
    fn add(&self, request: CalculationRequest) -> CalculationResult;
    fn subtract(&self, request: CalculationRequest) -> CalculationResult;
    fn multiply(&self, request: CalculationRequest) -> CalculationResult;
    fn divide(&self, request: CalculationRequest) -> Result<CalculationResult, String>;
}

#[derive(Serialize, Deserialize)]
pub struct CalculationResult {
    pub result: f64,
}