use std::{
    alloc::System,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{Json, error_handling::HandleError, extract::connect_info::Connected, http::Error};
use chrono::{
    DateTime, Datelike, FixedOffset, NaiveDate,
    format::{Parsed, parse_and_remainder},
};
use futures::future::err;
use influxdb2::{
    FromDataPoint,
    models::{Query, ast::dialect::DateTimeFormat},
};
use tokio::sync::watch::error;

use crate::repository::get_connection;

pub async fn get_cpu_info(
    start: &String,
    stop: &String,
) -> Result<Json<Vec<CpuInfo>>, Box<dyn std::error::Error>> {
    let connection = get_connection().await?;
    // let start_str = format!(
    //     "{}-{:02}-{:02}",
    //     start.year(),
    //     start.month0() + 1,
    //     start.day0() + 1
    // );
    // let stop_str = format!(
    //     "{}-{:02}-{:02}",
    //     stop.year(),
    //     stop.month0() + 1,
    //     stop.day0() + 1
    // );

    let qs = format!(
        "from(bucket: \"systemInfo\")
        |> range(start: {}, stop: {})
        |> aggregateWindow(every: 1h ,fn: median)
        |> filter(fn: (r) => r._value != 0)
        |> filter(fn: (r) => r._measurement == \"cpu\")",
        start, stop
    );
    let influx_query = Query::new(qs.to_string());

    // println!("res :::::: {influx_query:?}");
    let result = connection.query::<CpuInfo>(Some(influx_query)).await;
    println!("res :::::: {result:?}");
    match result {
        Ok(t) => {
            println!("{t:?}");
            return Ok(Json(t));
        }
        Err(e) => {
            println!("{e:?}");
            panic!("err{e:?}");
        }
    }
}

#[derive(Debug, FromDataPoint, serde::Serialize)]
pub struct CpuInfo {
    field: String,
    value: f64,
    time: DateTime<FixedOffset>,
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            field: "".to_string(),
            value: 0_f64,
            time: DateTime::<FixedOffset>::default(),
        }
    }
}
