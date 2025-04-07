use axum::Json;
use chrono::{
    Datelike, NaiveDate,
    format::{Parsed, parse_and_remainder},
};
use influxdb2::{FromDataPoint, models::Query};

use crate::repository::get_connection;

pub async fn get_cpu_info(
    start: NaiveDate,
    stop: NaiveDate,
) -> Result<Json<Vec<CpuInfo>>, Box<dyn std::error::Error>> {
    let connection = get_connection().await?;
    let start_str = format!(
        "{}-{}-{}",
        start.year(),
        start.month0() + 1,
        start.day0() + 1
    );
    let stop_str = format!("{}-{}-{}", stop.year(), stop.month0() + 1, stop.day0() + 1);

    let qs = format!(
        "from(bucket: \"systemInfo\")
        |> range(start: {}, stop: {})
        |> filter(fn: (r) => r._value != 0)
        |> filter(fn: (r) => r.__measurement == \"cpu\")",
        start_str, stop_str
    );
    let influx_query = Query::new(qs.to_string());

    let result = connection.query::<CpuInfo>(Some(influx_query)).await?;

    Ok(Json(result))
}

#[derive(Debug, FromDataPoint, serde::Serialize)]
pub struct CpuInfo {
    field: String,
    value: f64,
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            field: "".to_string(),
            value: 0_f64,
        }
    }
}
