mod products;

use axum::{http::StatusCode, routing::get, Router};
use products::product::Product;

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
    let mut product_service = products::ProductService::new();

    product_service.seed_products();

    let products = product_service.get_all().await;

    Ok(axum::Json(products))
}
