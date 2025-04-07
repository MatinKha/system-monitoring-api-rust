use std::fmt::Debug;

use crate::{repository::system_info::get_cpu_info, services::system_info::fetch_system_info};
use axum::{Json, Router, extract::Query, response::IntoResponse, routing::get};
use chrono::{Date, DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DateRange {
    start: String,
}

pub async fn get_cpu(date_range: Query<DateRange>) -> impl IntoResponse {
    println!("date_range {date_range:?}");
    let dt = NaiveDate::parse_from_str(&date_range.start, "%Y-%m-%d");
    if dt.is_err() {
        return Json("wrongData");
    }
    println!("{dt:?}");
    let info = get_cpu_info(dt, dt).await.unwrap();
    return info;
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
