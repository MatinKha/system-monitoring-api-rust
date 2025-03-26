use influxdb2::models::Query;

use crate::repository::get_connection;

pub async fn _get_system_info() -> Result<(), Box<dyn std::error::Error>> {
    let _connection = get_connection().await?;
    let qs = format!(
        "from(bucket: \"stock-prices\")
        |> range(start: -1w)
        |> filter(fn: (r) => r.ticker == \"{}\")
        |> last()
    ",
        "AAPL"
    );
    let _influx_query = Query::new(qs.to_string());

    Ok(())
}
