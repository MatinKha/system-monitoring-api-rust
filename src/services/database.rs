use axum::response::Result;
use std::env;

pub async fn connect() -> Result<()> {
    // Connect to the database.
    //  let (client, connection) =
    //       tokio_postgres::connect("host=localhost user=postgres password=1234", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });
    // let string = env::var("IG");
    //
    //  Now we can execute a simple statement that just returns its parameter.
    // let rows: Vec<Row> = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    Ok(())
}
