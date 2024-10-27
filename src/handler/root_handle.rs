use crate::error::AxumError;
use crate::response::Response as AxumResponse;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySql, QueryBuilder, Row};
use tracing::{error, info};
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateTicketQue {
    title: String,
    description: String,
    status: u8,
}

// 通过Path接收路由的参数
pub async fn get_handle(
    State(AppState { ref mysql_pool }): State<AppState>,
    Path(id): Path<u64>,
) -> Result<AxumResponse<Tickets>, AxumError> {
    let get_sql = "select * from `tickets` where `id`=?";
    let rep: Option<Tickets> = sqlx::query_as(get_sql)
        .bind(id)
        .fetch_optional(mysql_pool)
        .await
        .map_err(AxumError::Database)?;

    if let Some(tickets) = rep {
        Ok(AxumResponse::ok(Some(tickets)))
    } else {
        Err(AxumError::RecordNotFound)
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ResultData {
    pub message: String,
    pub data: u64,
}
// 通过Json接收json的请求数据
// 这里通过State(AppState{ref mysql_pool}):State<AppState> 这个方式解出来结构体中的数据
pub async fn create_handle(
    State(AppState { ref mysql_pool }): State<AppState>,
    Json(req): Json<CreateTicketQue>,
) -> Result<AxumResponse<ResultData>, AxumError> {
    info!("[create_handle] create req:{req:#?}");
    let insert_sql = "insert into `tickets` (`title`, `description`, `status`) values (?,?,?)";
    let rep = sqlx::query(insert_sql)
        .bind(req.title)
        .bind(req.description)
        .bind(req.status)
        .execute(mysql_pool)
        .await
        .map_err(AxumError::Database)?;
    let id = rep.last_insert_id();
    let mut headers = HeaderMap::new();
    headers.insert("key", HeaderValue::from_static("123"));
    let body = ResultData {
        message: "suc".to_string(),
        data: id,
    };
    Ok(AxumResponse::ok(Some(body)))
}

#[derive(Debug, Deserialize)]
pub struct Batch {
    item: Option<Vec<TicketItem>>,
}

#[derive(Debug, Deserialize)]
pub struct TicketItem {
    pub title: String,
    pub description: String,
    pub status: u8,
}

pub async fn batch_insert(
    State(AppState { ref mysql_pool }): State<AppState>,
    Json(req): Json<Batch>,
) -> Result<AxumResponse<ResultData>, AxumError> {
    let mut builder: QueryBuilder<MySql> =
        sqlx::query_builder::QueryBuilder::new("insert into tickets(title, description, status)");
    if let Some(item) = &req.item {
        builder.push_values(item, |mut b, t| {
            b.push_bind(&t.title)
                .push_bind(&t.description)
                .push_bind(&t.status);
        });
        let result = builder
            .build()
            .execute(mysql_pool)
            .await
            .map_err(AxumError::Database)?;

        let final_sql = builder.sql();
        let id = result.last_insert_id();
        let body = ResultData {
            message: "suc".to_string(),
            data: id,
        };
        return Ok(AxumResponse::ok(Some(body)));
    }
    Err(AxumError::RecordNotFound)
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicket {
    title: String,
    description: String,
    status: u8,
}

// 接收id和jsn数据
// curl --location --request PUT 'localhost:8081/tickets/2' \
// --header 'Content-Type: application/json' \
// --data '{
//     "title":"this is title",
//     "description":"this is description",
//     "status":0
// }'
pub async fn update_handle(
    State(AppState { ref mysql_pool }): State<AppState>,
    Path(id): Path<u64>,
    Json(req): Json<UpdateTicket>,
) -> Result<AxumResponse<ResultData>, AxumError> {
    let update_sql =
        "update `tickets` set `title` = ? , `description` = ? , `status` = ? where `id`=?;";
    let rep = sqlx::query(update_sql)
        .bind(req.title)
        .bind(req.description)
        .bind(req.status)
        .bind(id)
        .execute(mysql_pool)
        .await
        .map_err(AxumError::Database)?;
    let row_id = rep.rows_affected();
    let body = ResultData {
        message: "suc".to_string(),
        data: row_id,
    };
    Ok(AxumResponse::ok(Some(body)))
}

pub async fn delete_handle(
    State(AppState { ref mysql_pool }): State<AppState>,
    Path(id): Path<u64>,
) -> Result<AxumResponse<ResultData>, AxumError> {
    let delete_sql = "delete from `tickets` where id = ?";
    let rep = sqlx::query(delete_sql)
        .bind(id)
        .execute(mysql_pool)
        .await
        .map_err(AxumError::Database)?;
    let last_id = rep.rows_affected();
    let body = ResultData {
        message: "suc".to_string(),
        data: last_id,
    };
    Ok(AxumResponse::ok(Some(body)))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Tickets {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: bool,
    pub created_at: Option<chrono::DateTime<Local>>,
    pub updated_at: Option<chrono::DateTime<Local>>,
}

#[derive(sqlx::FromRow)]
struct Count {
    pub count: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ResultList {
    pub message: String,
    pub data: Vec<Tickets>,
    pub count: i64,
}
pub async fn list_handle(
    State(AppState { ref mysql_pool }): State<AppState>,
    Query(ListQuery { page, page_size }): Query<ListQuery>,
) -> Result<AxumResponse<ResultList>, AxumError> {
    let mut offset = 0;
    if let Some(p) = page {
        if let Some(p_size) = page_size {
            offset = (p - 1) * p_size;
        }
    }

    let list_sql = "select * from `tickets` where 1=1 LIMIT ? OFFSET ?";
    let rep: Vec<Tickets> = sqlx::query_as(list_sql)
        .bind(page_size)
        .bind(offset)
        .fetch_all(mysql_pool)
        .await
        .map_err(AxumError::Database)?;
    let total_sql = "SELECT COUNT(*) as count FROM `tickets` WHERE 1=1";
    let total: Count = sqlx::query_as(total_sql)
        .fetch_one(mysql_pool)
        .await
        .map_err(AxumError::Database)?;
    let c = total.count;

    Ok(AxumResponse::ok(Some(ResultList {
        message: "suc".to_owned(),
        data: rep,
        count: c,
    })))
}
