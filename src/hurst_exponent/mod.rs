use crate::utils::*;
use ndarray::prelude::*;
use ndarray::Array;

fn get_hurst_exponent_rs(arr: Array<f64, Ix1>) -> f64 {
    let ret = calculate_daily_returns(&arr);
    // n = np.round(len(ret) / 2 ** np.arange(0, 10)).astype(int)
    let n = (0..10).map(|exponent| arr.len() as f64 / f64::powi(2 as f64, exponent));
    return 1.0;
}
