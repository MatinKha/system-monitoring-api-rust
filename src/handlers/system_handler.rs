use std::fmt::Debug;

use crate::{repository::system_info::get_cpu_info, services::system_info::fetch_system_info};
use axum::{
    Json, Router,
    extract::Query,
    response::{self, IntoResponse, Response},
    routing::get,
};
use chrono::{Date, DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DateRange {
    start: String,
    stop: String,
}

pub async fn get_cpu(date_range: Query<DateRange>) -> Response {
    println!("date_range {date_range:?}");
    let d_start = NaiveDate::parse_from_str(&date_range.start, "%Y-%m-%d");
    let d_end = NaiveDate::parse_from_str(&date_range.stop, "%Y-%m-%d");

    let info = get_cpu_info(d_start, d_end).await.unwrap();
    return info.into_response();
}

pub async fn get_ram() -> impl IntoResponse {
    // Placeholder: Add logic to restart system safely
    let json = Json("skldfjasdlfj");
    return json;
}

pub fn routes() -> Router {
    Router::new()
        .route("/cpu", get(get_cpu))
        .route("/ram", get(get_ram))
}
