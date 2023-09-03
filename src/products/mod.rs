pub mod product;

use product::Product;
use rust_decimal::{prelude::FromPrimitive, Decimal};

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
        });
    }

    // Create a function that gets all product from an in-memory list and then returns them
    pub async fn get_all(self) -> Vec<Product> {
        self.products
    }
}
