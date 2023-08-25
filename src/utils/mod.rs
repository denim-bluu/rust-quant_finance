use ndarray::prelude::*;
use ndarray::Array;

pub fn calculate_daily_returns(arr: &Array<f64, Ix1>) -> Array<f64, Ix1> {
    let v1 = &arr.slice(s![..-1]);
    let v2 = &arr.slice(s![1..]);
    (v2 - v1) / v1
}

pub fn calculate_rolling_mean(arr: &Array<f64, Ix1>, window: usize) -> Array<f64, Ix1> {
    let mut roll_mean = Array::zeros(arr.len());

    for i in 0..arr.len() {
        if i < window {
            roll_mean[i] = arr.slice(s![..=i]).mean().unwrap();
        } else {
            roll_mean[i] = arr.slice(s![i - window + 1..=i]).mean().unwrap();
        }
    }

    roll_mean
}
pub fn calculate_rolling_std(
    arr: &Array<f64, Ix1>,
    window: usize,
    ddof: Option<f64>,
) -> Array<f64, Ix1> {
    const DDOF: f64 = 1.0;
    let mut roll_mean = Array::zeros(arr.len());

    for i in 0..arr.len() {
        if i < window {
            roll_mean[i] = arr.slice(s![..=i]).std(ddof.unwrap_or(DDOF));
        } else {
            roll_mean[i] = arr.slice(s![i - window + 1..=i]).std(ddof.unwrap_or(DDOF));
        }
    }

    roll_mean
}
