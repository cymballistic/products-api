use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub price: Decimal,
    pub images: Vec<String>,
    pub sizes: Vec<String>,
}
