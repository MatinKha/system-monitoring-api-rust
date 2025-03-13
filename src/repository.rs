use axum::response::Result;
use hyper::client;
use influxdb2::Client;
use std::env;

pub async fn connect() -> Result<()> {
    let host = std::env::var("INFLUXDB_HOST").unwrap();
    let org = std::env::var("INFLUXDB_ORG").unwrap();
    let token = std::env::var("INFLUXDB_TOKEN").unwrap();
    let client = Client::new(host, org, token);
    Ok(())
}
