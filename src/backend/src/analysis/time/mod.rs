pub mod frequency;
pub mod date;

pub fn insert_bin(bin_subdiv: u64, bin_max: u64, values: &Vec<(u64, f64)>) -> Option<Vec<(u64, f64)>> {
    // bin_max in unlikely to be divisible by bin_subdiv,
    // so the sum of all bins can be smaller than bin_max.
    // the rest remainder of the division needs to be accounted for
    let individual_bin_size = bin_max / bin_subdiv;

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
