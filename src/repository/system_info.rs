use futures::stream;
use influxdb2::models::DataPoint;
use sysinfo::System;

use crate::repository::get_connection;

pub async fn write_system_info() -> Result<(), Box<dyn std::error::Error>> {
    let connection = match get_connection().await {
        Ok(c) => c,
        Err(e) => {
            panic!();
        }
    };
    let sys = System::new_all();
    let points = vec![
        DataPoint::builder("cpu")
            .tag("host", "server01")
            .tag("region", "us-west")
            .field("usage", sys.global_cpu_usage().to_string())
            .build()?,
    ];
    connection.write("systemInfo", stream::iter(points)).await?;
    Ok(())
}
