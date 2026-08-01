use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::analysis::time::insert_bin;

pub fn group(
    data: &Vec<(DateTime<Tz>, f64)>,
    resolution: u64
) -> HashMap<u64, f64> {
    let timestamps = data.iter().map(|f| f.0.timestamp()).collect::<Vec<i64>>();

    let min = timestamps.iter().min().unwrap_or(&0);
    let max = timestamps.iter().max().unwrap_or(&0);

    let elapsed = (max - min) as f64;

    let bin_count = resolution;
    let bin_size = (elapsed / (resolution as f64)) as u64;

    let mut values: Vec<(u64, f64)> = Vec::new();

    for (datetime, duration) in data {
        let datetime = datetime.timestamp() - min;

        values.push((datetime as u64, *duration));
    }

    let bins = insert_bin(bin_count, bin_size, &values);

    return bins.into_iter().map(|t| (t.0 + (*min as u64), t.1)).collect::<HashMap<u64, f64>>();
}
