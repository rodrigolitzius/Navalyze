use std::collections::HashMap;

use chrono::{DateTime, Timelike};
use chrono_tz::Tz;

use crate::analysis::time::insert_bin;

pub fn group(
    data: &Vec<(DateTime<Tz>, f64)>,
    resolution: u64
) -> HashMap<u64, f64> {
    let bin_size = (24*60*60) / resolution;

    let mut values: Vec<(u64, f64)> = Vec::new();

    for (datetime, duration) in data {
        let datetime = datetime.num_seconds_from_midnight();

        values.push((datetime as u64, *duration));
    }

    return insert_bin(resolution, bin_size, &values);
}
