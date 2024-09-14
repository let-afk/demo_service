use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

type OrderCache = Arc<RwLock<HashMap<String, Order>>>;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cache: OrderCache = Arc::new(RwLock::new(HashMap::new()));

    cache
        .write()
        .await
        .insert("b563feb7b2b84b6test".to_string(), create_test_order());

    info!("The test order has been added to the cache");

    let app = Router::new()
        .route("/orders/:order_uid", get(get_order))
        .with_state(cache.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_order(
    Path(order_uid): Path<String>,
    State(state): State<OrderCache>,
) -> impl IntoResponse {
    let cache = state.read().await;

    info!("An order request with a UID was received: {}", order_uid);

    if let Some(order) = cache.get(&order_uid) {
        (StatusCode::OK, serde_json::to_string(order).unwrap())
    } else {
        info!("Order with UID: {} not found", order_uid);
        (
            StatusCode::NOT_FOUND,
            json!({"error": "Order not found"}).to_string(),
        )
    }
}

fn create_test_order() -> Order {
    Order {
        order_uid: "b563feb7b2b84b6test".to_string(),
        track_number: "WBILMTESTTRACK".to_string(),
        entry: "WBIL".to_string(),
        delivery: Delivery {
            name: "Test Testov".to_string(),
            phone: "+9720000000".to_string(),
            zip: "2639809".to_string(),
            city: "Kiryat Mozkin".to_string(),
            address: "Ploshad Mira 15".to_string(),
            region: "Kraiot".to_string(),
            email: "test@gmail.com".to_string(),
        },
        payment: Payment {
            transaction: "b563feb7b2b84b6test".to_string(),
            request_id: "".to_string(),
            currency: "USD".to_string(),
            provider: "wbpay".to_string(),
            amount: 1817,
            payment_dt: 1637907727,
            bank: "alpha".to_string(),
            delivery_cost: 1500,
            goods_total: 317,
            custom_fee: 0,
        },
        items: vec![Item {
            chrt_id: 9934930,
            track_number: "WBILMTESTTRACK".to_string(),
            price: 453,
            rid: "ab4219087a764ae0btest".to_string(),
            name: "Mascaras".to_string(),
            sale: 30,
            size: "0".to_string(),
            total_price: 317,
            nm_id: 2389212,
            brand: "Vivienne Sabo".to_string(),
            status: 202,
        }],
        locale: "en".to_string(),
        internal_signature: "".to_string(),
        customer_id: "test".to_string(),
        delivery_service: "meest".to_string(),
        shardkey: "9".to_string(),
        sm_id: 99,
        date_created: "2021-11-26T06:22:19Z".to_string(),
        oof_shard: "1".to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: u32,
    date_created: String,
    oof_shard: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: u32,
    payment_dt: u64,
    bank: String,
    delivery_cost: u32,
    goods_total: u32,
    custom_fee: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    chrt_id: u32,
    track_number: String,
    price: u32,
    rid: String,
    name: String,
    sale: u32,
    size: String,
    total_price: u32,
    nm_id: u32,
    brand: String,
    status: u32,
}
