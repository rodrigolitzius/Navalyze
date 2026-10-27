use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::analysis::time::insert_bin;

pub fn group(
    data: &Vec<(DateTime<Tz>, f64)>,
    resolution: u64
) -> HashMap<u64, f64> {
    let timestamps: Vec<i64> = data.iter().map(|f| f.0.timestamp()).collect();

    let min = timestamps.iter().min().unwrap_or(&0);
    let max = timestamps.iter().max().unwrap_or(&0);

    let elapsed = max - min;

    let values: Vec<(u64, f64)> = data.iter().map(|d| {
        let timestamp = (d.0.timestamp() - min) as u64;
        (timestamp, d.1)
    }).collect();

    return insert_bin(resolution, elapsed as u64, &values)
        .unwrap_or(values)
        .into_iter()
        .map(|t| (t.0 + (*min as u64), t.1))
        .collect();
}
