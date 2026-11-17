pub mod daily;
pub mod date;
pub mod weekly;

use std::collections::HashMap;

use chrono::DateTime;
use chrono_tz::Tz;

use crate::handlers::extract::GraphTypeEnum;

pub fn insert_bin(bin_subdiv: u64, bin_max: u64, values: &Vec<(u64, f64)>) -> Option<Vec<(u64, f64)>> {
    // bin_max in unlikely to be divisible by bin_subdiv,
    // so the sum of all bins can be smaller than bin_max.
    // the remainder of the division needs to be accounted for
    let individual_bin_size = (bin_max + 1) / bin_subdiv;

    // Bin is too big
    if individual_bin_size == 0 {
        return None
    }

    let mut bins: Vec<f64> = vec![0.0; bin_subdiv as usize];

    for (value, add) in values {
        debug_assert!(!(*value > bin_max));

        let bin_i = value / individual_bin_size;

        // Ignores the remainder mentioned above
        // This is unfortunately lost data
        if bin_i < bin_subdiv {
            bins[bin_i as usize] += add;
        }
    }

    return Some(bins
        .into_iter()
        .enumerate()
        .map(|b| {((b.0 as u64) * individual_bin_size, b.1)})
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
