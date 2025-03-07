use std::fmt;
use sysinfo::System;
use tokio_postgres;

pub fn fetch_system_info() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();
    format!(
        "OS: {}, CPU: {}, Memory: {} MB",
        System::name().unwrap_or("Unknown".to_string()),
        sys.global_cpu_usage(),
        sys.total_memory() / 1024
    )
}
