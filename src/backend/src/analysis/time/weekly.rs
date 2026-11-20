use std::collections::HashMap;

use chrono::{DateTime, Datelike};
use chrono_tz::Tz;

use crate::analysis::time::insert_bin;

pub fn group(
    data: &Vec<(DateTime<Tz>, f64)>
) -> HashMap<u64, f64> {
    let values: Vec<(u64, f64)> = data.iter().map(|d| {
        let weekday = d.0.weekday() as u64;
        (weekday, d.1)
    }).collect();

    return insert_bin(7.0, 6.0, &values)
        .unwrap_or(values)
        .into_iter().collect();
}
