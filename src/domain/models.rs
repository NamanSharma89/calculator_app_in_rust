// src/domain/models.rs

// src/domain/calculation_request.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CalculationRequest {
    pub a: i32,
    pub b: i32,
}