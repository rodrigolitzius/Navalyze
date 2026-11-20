pub mod daily;
pub mod date;
pub mod weekly;

use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::handlers::extract::GraphTypeEnum;

pub fn insert_bin(bin_subdiv: f64, bin_max: f64, values: &Vec<(u64, f64)>) -> Option<Vec<(u64, f64)>> {
    // I don't know why the + 1 has to be here, but it does
    let individual_bin_size = (bin_max + 1.0) / bin_subdiv;

    if individual_bin_size < 1.0 {
        return None
    }

    let mut bins: Vec<f64> = vec![0.0; bin_subdiv as usize];

    for (value, add) in values {
        let bin_i = (*value as f64) / individual_bin_size;

        bins[bin_i as usize] += add;
    }

    return Some(bins
        .into_iter()
        .enumerate()
        .map(|b| {(((b.0 as f64) * individual_bin_size) as u64, b.1)})
        .collect());
}

pub fn group_type(
    data: &Vec<(DateTime<Tz>, f64)>,
    resolution: Option<u64>,
    artist_type: &GraphTypeEnum
) -> HashMap<u64, f64> {
    return match artist_type {
        GraphTypeEnum::AllTime => date::group(data, resolution.expect("Resolution should not be None")),
        GraphTypeEnum::Daily => daily::group(data, resolution.expect("Resolution should not be None")),
        GraphTypeEnum::Weekly => weekly::group(data)
    }
}
