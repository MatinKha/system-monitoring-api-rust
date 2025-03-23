use influxdb2::{Client, RequestError};
use std::env;

pub mod system_info;

pub async fn get_connection() -> Result<Client, RequestError> {
    let host = env::var("INFLUXDB_HOST").unwrap();
    let org = env::var("INFLUXDB_ORG").unwrap();
    let token = env::var("INFLUXDB_TOKEN").unwrap();
    let client: Client = Client::new(host, org, token);
    println!("help meP{}", client.ready().await?.to_string());
    Ok(client)
}
