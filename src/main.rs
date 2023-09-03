use parking_lot::RwLock;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use warp::{http, Filter};

type Products = Vec<Product>;

#[tokio::main]
async fn main() {
    let product_service = ProductService::new();
    let products_service_filter = warp::any().map(move || product_service.clone());

    let get_products = warp::get()
        .and(warp::path("products"))
        .and(warp::path::end())
        .and(products_service_filter.clone())
        .and_then(get_all_products);

    let add_product = warp::post()
        .and(warp::path("products"))
        .and(warp::path::end())
        .and(validate_post_json())
        .and(products_service_filter.clone())
        .and_then(add_product);

    let delete_product = warp::delete()
        .and(warp::path("products"))
        .and(warp::path::end())
        .and(validate_delete_json())
        .and(products_service_filter.clone())
        .and_then(delete_product);

    let routes = add_product.or(get_products).or(delete_product);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn validate_post_json() -> impl Filter<Extract = (ProductPost,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn validate_delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn get_all_products(
    product_service: ProductService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let products = product_service.products.read();

    Ok(warp::reply::json(&*products))
}

async fn add_product(
    product_in: ProductPost,
    product_service: ProductService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let product: Product = Product {
        id: Uuid::new_v4(),
        name: product_in.name,
        brand: product_in.brand,
        price: product_in.price,
        images: product_in.images,
        sizes: product_in.sizes,
    };

    product_service.products.write().push(product);
    Ok(warp::reply::with_status(
        "Added product",
        http::StatusCode::CREATED,
    ))
}

async fn delete_product(
    id: Id,
    product_service: ProductService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let index_to_remove = product_service
        .products
        .read()
        .iter()
        .position(|x| x.id == id.id)
        .unwrap();

    product_service.products.write().remove(index_to_remove);

    Ok(warp::reply::with_status(
        "Removed product",
        http::StatusCode::OK,
    ))
}

#[derive(Clone)]
pub struct ProductService {
    products: Arc<RwLock<Products>>,
}

impl ProductService {
    pub fn new() -> ProductService {
        ProductService {
            products: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub brand: String,
    pub price: Decimal,
    pub images: Vec<String>,
    pub sizes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductPost {
    pub name: String,
    pub brand: String,
    pub price: Decimal,
    pub images: Vec<String>,
    pub sizes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Id {
    pub id: Uuid,
}
