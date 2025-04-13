// src/app/calculator_app.rs

#[derive(Debug, PartialEq)]
pub struct CalculatorApp {}

impl CalculatorApp {
    pub fn new() -> Self {
        CalculatorApp {}
    }

    pub fn add(&self, a: i32, b: i32) -> Result<i32, String> {
        Ok(a + b)
    }

    pub fn subtract(&self, a: i32, b: i32) -> Result<i32, String> {
        Ok(a - b)
    }

    pub fn multiply(&self, a: i32, b: i32) -> Result<i32, String> {
        Ok(a * b)
    }

    pub fn divide(&self, a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}