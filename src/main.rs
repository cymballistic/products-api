use axum::{http::StatusCode, routing::get, Router};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_all));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_all() -> Result<axum::Json<Vec<Product>>, (StatusCode, String)> {
    let mut product_service = ProductService::new();

    product_service.seed_products();

    let products = product_service.get_all().await;

    Ok(axum::Json(products))
}

pub struct ProductService {
    products: Vec<Product>,
}

impl ProductService {
    // constructor
    pub fn new() -> ProductService {
        ProductService {
            products: Vec::new(),
        }
    }

    pub fn seed_products(&mut self) {
        self.products.push(Product {
            id: 1,
            name: "Product 1".to_string(),
            brand: "Brand 1".to_string(),
            price: Decimal::from_f32(100.0).unwrap(),
            images: vec![
                "https://randomwordgenerator.com/img/picture-generator/paprika-4336024_640.jpg".to_string(),
                "https://randomwordgenerator.com/img/picture-generator/53e3d6434d5aaa14f1dc8460962e33791c3ad6e04e507440762e7ad3964acc_640.jpg".to_string()
            ],
            sizes: vec![
                "S".to_string(),
                "M".to_string(),
                "L".to_string(),
                "XL".to_string(),
                "XXL".to_string(),]
        });
        self.products.push(Product {
            id: 2,
            name: "Product 2".to_string(),
            brand: "Brand 2".to_string(),
            price: Decimal::from_f32(200.0).unwrap(),
            images: vec![
                "https://randomwordgenerator.com/img/picture-generator/paprika-4336024_640.jpg".to_string(),
                "https://randomwordgenerator.com/img/picture-generator/53e3d6434d5aaa14f1dc8460962e33791c3ad6e04e507440762e7ad3964acc_640.jpg".to_string()
            ],
            sizes: vec!["S".to_string(), "L".to_string(), "XXL".to_string()],
        });
        self.products.push(Product {
            id: 3,
            name: "Product 3".to_string(),
            brand: "Brand 3".to_string(),
            price: Decimal::from_f32(300.0).unwrap(),
            images: vec![
                "https://randomwordgenerator.com/img/picture-generator/paprika-4336024_640.jpg".to_string(),
                "https://randomwordgenerator.com/img/picture-generator/53e3d6434d5aaa14f1dc8460962e33791c3ad6e04e507440762e7ad3964acc_640.jpg".to_string()
            ],
            sizes: vec![
                "S".to_string(),
                "M".to_string(),
                "L".to_string(),
                "XL".to_string(),
                "XXL".to_string(),
            ],
        });
    }

    // Create a function that gets all product from an in-memory list and then returns them
    pub async fn get_all(self) -> Vec<Product> {
        self.products
    }
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub price: Decimal,
    pub images: Vec<String>,
    pub sizes: Vec<String>,
}
