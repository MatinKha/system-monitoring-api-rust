use std::{any, collections::BTreeMap, string};

use axum::Json;
use influxdb2::{FromDataPoint, FromMap, models::Query};

use crate::repository::get_connection;

pub async fn get_cpu_info() -> Result<(), Box<dyn std::error::Error>> {
    let connection = get_connection().await?;
    let qs = format!(
        "from(bucket: \"systemInfo\")
        |> range(start: -1w)"
    );
    let influx_query = Query::new(qs.to_string());

    println!("help : {influx_query:?} \n\n");
    let result = connection.query::<CpuInfo>(Some(influx_query)).await?;
    println!("help : {result:?} \n\n");

    for item in result {
        println!("help : {item:?}");
    }
    Ok(())
}
#[derive(Debug, FromDataPoint)]
struct CpuInfo {
    field: f64,
    tag: String,
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            field: 0_f64,
            tag: "".to_string(),
        }
    }
}
