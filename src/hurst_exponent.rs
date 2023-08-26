use crate::math::*;
use polyfit_rs::polyfit_rs;

/// The function `calculate_rs` calculates the R/S ratio, which is the range divided by the
/// standard deviation, for a given array of numbers.
///
/// Arguments:
///
/// * `x`: An array of f64 values.
///
/// Returns:
///
/// The function `calculate_rs` returns a `f64` value, which is the result of dividing `r`
/// by `s`.
pub fn calculate_rs(x: &[f64]) -> f64 {
    let mean_adjusted = x.iter().map(|v| v - mean(x)).collect();
    let cumul_mean_adjusted = cumulative_sum(&mean_adjusted);
    let r = array_max(&cumul_mean_adjusted) - array_min(&cumul_mean_adjusted);
    let s = standard_deviation(x, 0.0);
    r / s
}

/// The `calculate_hurst_exp` function calculates the Hurst exponent for a given set of
/// returns.
///
/// Arguments:
///
/// * `ret`: The `ret` parameter is a slice of `f64` values representing the returns of a
/// financial asset over a certain period of time.
///
/// Returns:
///
/// The function `calculate_hurst_exp` returns a `f64` value, which represents the
/// calculated Hurst exponent.
pub fn calcualte_hurst_exp(ret: &[f64]) -> f64 {
    let n: Vec<f64> = (0..10)
        .map(|x| (ret.len() as f64 / f64::powf(2., x as f64)).round())
        .collect();

    let mut y: Vec<f64> = Vec::with_capacity(n.len());
    let x: Vec<f64> = n.clone();

    for &x in n.iter() {
        let mut rs: Vec<f64> = Vec::new();
        let t = ret.len() / x as usize;

        for j in 0..t {
            let sub_ts = &ret[j * x as usize..(j + 1) * x as usize];
            rs.push(calculate_rs(sub_ts));
        }
        y.push(mean(&rs));
    }
    let log_x: Vec<f64> = x.iter().map(|v| v.ln()).collect();
    let log_y: Vec<f64> = y.iter().map(|v| v.ln()).collect();
    polyfit_rs::polyfit(&log_x, &log_y, 1).expect("Something went wrong")[1]
}
