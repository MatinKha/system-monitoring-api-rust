// use std::{
//     collections::{BTreeMap, HashMap, hash_map},
//     string,
// };
//
// use axum::{Json, http::Result};
// use chrono::{DateTime, Duration, FixedOffset};
// use erased_serde::Serialize;
// use influxdb2::models::{Query, ast::string_literal};
// use ordered_float::OrderedFloat;
// use serde_json::to_string;
//
// use crate::repository::get_connection;
//
// pub async fn raw_system_query() {
//     let qs = "from(bucket: \"systemInfo\")
//         |> range(start: -1h,)
//         |> filter(fn: (r) => r[\"_measurement\"] == \"top_proc\")";
//     let connection = get_connection().await.unwrap();
//     let influx_query = Query::new(qs.to_string());
//
//     let result = connection.query_raw(Some(influx_query)).await.unwrap();
//     for ele in result {
//         for ele in ele.values {
//             let e = toString(ele.1.);
//             println!("{e}")
//         }
//     }
//
//     //
//     // match result {
//     //     Ok(t) => {
//     //         // println!("{t:?}");
//     //         return Ok(Json(t));
//     //     }
//     //     Err(e) => {
//     //         println!("{e:?}");
//     //         panic!("err{e:?}");
//     //     }
//     // }
// }
// fn toString(elm: influxdb2_structmap::value::Value) -> String {
//     elm.to_string()
// }
// // #[derive(Serialize)]
// // pub enum SerializableValue1 {
// //     Unknown,
// //     String(String),
// //     Double(OrderedFloat<f64>),
// //     Bool(bool),
// //     Long(i64),
// //     UnsignedLong(u64),
// //     Duration(Duration),
// //     Base64Binary(Vec<u8>),
// //     TimeRFC(DateTime<FixedOffset>),
// }
