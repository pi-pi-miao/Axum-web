use axum::http::HeaderName;
use axum::routing::{delete, post, put};
use axum::{body::Body, http::Request, routing::get, Router};
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
mod pool;
use crate::handler::root_handle::{
    batch_insert, create_handle, delete_handle, get_handle, list_handle, update_handle,
};
use sqlx::{MySql, Pool};
use tracing::info;

#[derive(Debug, Clone)]
struct AppState {
    mysql_pool: Pool<MySql>,
}
mod error;
mod handler;
mod logger;
mod response;
mod utils;

#[tokio::main]
async fn main() {
    let (file_guard, std_out_guard) = logger::logger::init();
    let pool = pool::connect_pool().await;
    let state = AppState { mysql_pool: pool };
    let app = Router::new()
        .route("/tickets/:id", get(get_handle))
        .route("/tickets", post(create_handle))
        .route("/tickets/:id", put(update_handle))
        .route("/tickets/:id", delete(delete_handle))
        .route("/tickets", get(list_handle))
        .route("/tickets/batch", post(batch_insert))
        .layer(
            // 官方推荐在ServiceBuilder上一次性载入
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                //  压缩中间件，默认情况下，只有内容长度大于32的时候才会进行压缩
                .layer(CompressionLayer::new())
                // 超时时间是200ms 超时中间件 如果服务端响应超时就返回408状态码
                .layer(TimeoutLayer::new(Duration::from_millis(200)))
                // 需要设置一个请求头的键名，一般叫x-request-id
                .layer(SetRequestIdLayer::new(
                    HeaderName::from_static("x-request-id"),
                    MakeRequestUuid,
                ))
                // 默认情况下不放行，所以需要根据自己需求设置必要的允许规则。 设置CORS请求头
                .layer(
                    CorsLayer::new()
                        .allow_methods(http::Method::GET)
                        .allow_origin(Any),
                ),
        )
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Server is listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    logger::logger::drop_guard(file_guard, std_out_guard);
}
