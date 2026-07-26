pub mod frequency;
pub mod artist;

use std::collections::HashMap;

use crate::handlers::extract::Range;

pub fn insert_bin(bin_count: u64, bin_size: u64, values: &Vec<u64>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();

    for value in values {
        for bin in 0..bin_count {
            let range = Range {
                start: bin*bin_size,
                end: (bin+1)*bin_size
            };

            let key = range.start;

            match result.get(&key) {
                Some(_) => {},
                None => {let _ = result.insert(key, 0);}
            };

            if range.contains(&value) {
                match result.get_mut(&key) {
                    Some(v) => *v += 1,
                    None => {let _ = result.insert(key, 1);} // This should never match
                };
            }
        }
    }

    return result;
}
