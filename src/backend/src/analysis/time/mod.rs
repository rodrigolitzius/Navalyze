pub mod frequency;
pub mod date;

use std::collections::HashMap;

use crate::handlers::extract::Range;

pub fn insert_bin(bin_count: u64, bin_size: u64, values: &Vec<(u64, f64)>) -> HashMap<u64, f64> {
    let mut result: HashMap<u64, f64> = HashMap::new();

    for (value, add) in values {
        for bin in 0..bin_count {
            let range = Range {
                start: bin*bin_size,
                end: (bin+1)*bin_size
            };

            let key = range.start;

            match result.get(&key) {
                Some(_) => {},
                None => {let _ = result.insert(key, 0.0);}
            };

            if range.contains(&value) {
                match result.get_mut(&key) {
                    Some(v) => *v += *add,
                    None => {let _ = result.insert(key, *add);} // This should never match
                };
            }
        }
    }

    return result;
}
