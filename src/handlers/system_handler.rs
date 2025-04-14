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
pub struct DateRange {
    start: String,
    stop: String,
}

pub async fn get_cpu(date_range: Query<DateRange>) -> Response {
    println!("date_range {date_range:?}");
    // let d_start = NaiveDate::parse_from_str(&date_range.start, "%Y-%m-%d");
    // let d_end = NaiveDate::parse_from_str(&date_range.stop, "%Y-%m-%d");

    // if d_start.is_ok() && d_end.is_ok() {
    let info = get_cpu_info(&date_range.start, &date_range.stop).await;
    match info {
        Ok(t) => return t.into_response(),
        Err(_e) => {}
    }
    return Json("error on get_cpu").into_response();
    // }
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
