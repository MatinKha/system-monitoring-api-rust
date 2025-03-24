use futures::stream;
use influxdb2::models::DataPoint;
use sysinfo::System;

use crate::repository::get_connection;

pub async fn get_system_info() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
