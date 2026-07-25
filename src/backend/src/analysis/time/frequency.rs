use std::collections::HashMap;

use chrono::{DateTime, Timelike};
use chrono_tz::Tz;

use crate::handlers::extract::Range;

pub fn group(
    datetimes: Vec<&DateTime<Tz>>,
    resolution_sec: u64
) -> HashMap<u64, u64> {
    let bins_len = (24*60*60) / resolution_sec;

    let mut result: HashMap<u64, u64> = HashMap::new();

    for datetime in datetimes {
        let second = datetime.num_seconds_from_midnight() as u64;

        for v in 0..bins_len {
            let range = Range {
                start: v*resolution_sec,
                end: (v+1)*resolution_sec
            };

            if range.contains(&second) {
                match result.get_mut(&v) {
                    Some(v) => *v += 1,
                    None => {let _ = result.insert(v, 1);}
                };
            }
        }
    }

    return result;
}
