use tokio;

mod repository;
mod tasks;

#[tokio::main]
async fn main() {
    // loop every second
    let forever = tokio::task::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            interval.tick().await;
            tasks::log_ram::log_ram().await;
        }
    });

    let forever2 = tokio::task::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs_f64(0.5));

        loop {
            interval.tick().await;
            tasks::log_cpu::log_cpu().await;
        }
    });

    forever2.await;
    forever.await;
}

async fn update(message: &str) -> () {
    println!("log from update message: {}", message);
}
