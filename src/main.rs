/// Calculates the weighted average of a set of values.
///
/// # Arguments
///
/// * `values` - A slice of `f64` representing the values.
/// * `weights` - A slice of `f64` representing the weights corresponding to each value.
///
/// # Returns
///
/// * `Option<f64>` - Returns `Some(weighted_average)` if calculation is possible,
///   otherwise `None` (e.g., if the total weight is zero or slices have different lengths).
///
/// # Example
///
/// ```
/// let values = [3.0, 5.0, 2.0];
/// let weights = [2.0, 3.0, 5.0];
/// let average = weighted_average(&values, &weights);
/// assert_eq!(average, Some(3.4));
/// ```
use rayon::prelude::*;

/// Calculates the weighted average using parallel processing.
pub fn weighted_average_parallel(values: &[f64], weights: &[f64]) -> Option<f64> {
    if values.len() != weights.len() {
        eprintln!("Error: The length of values and weights must be the same.");
        return None;
    }

    // Use parallel iterators to compute weighted_sum and total_weight
    let (weighted_sum, total_weight) = values
        .par_iter()
        .zip(weights.par_iter())
        .map(|(&value, &weight)| (value * weight, weight))
        .reduce(
            || (0.0, 0.0),
            |acc, x| (acc.0 + x.0, acc.1 + x.1),
        );

    if total_weight == 0.0 {
        eprintln!("Error: Total weight is zero, cannot compute weighted average.");
        return None;
    }

    Some(weighted_sum / total_weight)
}

fn main() {
    let values: Vec<f64> = (1..=1_000_000).map(|x| x as f64).collect();
    let weights: Vec<f64> = (1..=1_000_000).map(|x| x as f64).collect();

    match weighted_average_parallel(&values, &weights) {
        Some(avg) => println!("The weighted average is: {}", avg),
        None => println!("Failed to calculate weighted average."),
    }
}