use std::collections::HashMap;

use chrono::{DateTime, Timelike};
use chrono_tz::Tz;

use crate::analysis::time::insert_bin;

pub fn group(
    data: &Vec<(DateTime<Tz>, f64)>,
    resolution: u64
) -> HashMap<u64, f64> {
    let values: Vec<(u64, f64)> = data.iter().map(|d| {
        let time = d.0.num_seconds_from_midnight() as u64;
        (time, d.1)
    }).collect();

    return insert_bin(resolution, 24*60*60, &values)
        .unwrap_or(values)
        .into_iter().collect();
}
