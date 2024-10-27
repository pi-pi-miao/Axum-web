use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

// mysql pool connection
pub async fn connect_pool() -> Pool<MySql> {
    MySqlPoolOptions::new()
        .connect("mysql://root:123456@127.0.0.1:3306/axum_example")
        .await
        .unwrap()
}
