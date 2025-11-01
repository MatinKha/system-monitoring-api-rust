use std::{collections::HashMap, fmt::Debug};

use crate::{
    repository::system_info::{
        CpuInfo, MemoryInfo, get_cpu_info, get_query_with_daterange, get_query_with_last,
    },
    services::system_info::fetch_system_info,
};
use axum::{
    Json, Router,
    extract::Query,
    response::{self, IntoResponse, Response},
    routing::get,
};
use chrono::{Date, DateTime, NaiveDate, Utc};
use influxdb2::models::DataPoint;
use num_traits::ToPrimitive;
use serde::Deserialize;
use sysinfo::{CpuRefreshKind, RefreshKind};

pub async fn get_cpu(query: Query<HashMap<String, String>>) -> Response {
    let bucket = String::from("systemInfo");
    println!("date_range {query:?}");

    // if d_start.is_ok() && d_end.is_ok() {
    if query.contains_key("start") && query.contains_key("stop") {
        let info =
            get_cpu_info::<CpuInfo>(get_query_with_daterange(&query["start"], &query["stop"]))
                .await;

        match info {
            Ok(t) => return t.into_response(),
            Err(_e) => {
                println!("{_e}");
            }
        }
    } else if query.contains_key("last") {
        if query["measurement"] == "memory" {
            let info = get_cpu_info::<MemoryInfo>(get_query_with_last(
                &bucket,
                &query["measurement"],
                &query["field"],
                &query["last"],
            ))
            .await;
            match info {
                Ok(t) => return t.into_response(),
                Err(_e) => {
                    println!("{_e}");
                }
            }
        }
        if query["measurement"] == "cpu" {
            let info = get_cpu_info::<CpuInfo>(get_query_with_last(
                &bucket,
                &query["measurement"],
                &query["field"],
                &query["last"],
            ))
            .await;
            match info {
                Ok(t) => return t.into_response(),
                Err(_e) => {
                    println!("{_e}");
                }
            }
        } else {
            let info = get_cpu_info::<CpuInfo>(get_query_with_last(
                &bucket,
                &query["measurement"],
                &query["field"],
                &query["last"],
            ))
            .await;
            match info {
                Ok(t) => return t.into_response(),
                Err(_e) => {
                    println!("{_e}");
                }
            }
        }
    }
    return Json("error on get_cpu").into_response();
    // }
}

pub async fn get_ram() -> impl IntoResponse {
    // Placeholder: Add logic to restart system safely
    let json = Json("skldfjasdlfj");
    return json;
}

pub async fn get_cpu_current() -> impl IntoResponse {
    let mut sys = sysinfo::System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    println!("logging cpu");

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let mut vec: Vec<_> = vec![];
    for cpu in cpus {
        vec.push(cpu.cpu_usage());
    }
    Json(vec)
}

pub fn routes() -> Router {
    Router::new()
        .route("/current", get(get_cpu_current))
        .route("/range", get(get_cpu))
        .route("/ram", get(get_ram))
}
