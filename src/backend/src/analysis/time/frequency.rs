use std::collections::HashMap;

use chrono::{DateTime, Timelike};
use chrono_tz::Tz;

use crate::analysis::time::insert_bin;

pub fn group(
    datetimes: Vec<&DateTime<Tz>>,
    resolution: u64
) -> HashMap<u64, u64> {
    let bin_size = (24*60*60) / resolution;

    return insert_bin(resolution, bin_size, &datetimes.iter().map(|d| d.num_seconds_from_midnight() as u64).collect());
}
