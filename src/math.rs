use nalgebra as na;

pub fn mean(x: &[f64]) -> f64 {
    let sum: f64 = x.iter().sum();
    let n: f64 = x.len() as f64;
    sum / n
}

/// Compute the standard deviation of a slice
pub fn standard_deviation(x: &[f64], ddof: f64) -> f64 {
    let mean_x: f64 = mean(x);
    let sum_x_minus_mean: f64 = x.iter().map(|a| (a - mean_x).powi(2)).sum();
    (sum_x_minus_mean / (x.len() as f64 - ddof)).sqrt()
}

pub fn calculate_daily_returns(x: &Vec<f64>) -> Vec<f64> {
    let v_lag = &x[..x.len() - 1];
    let v = &x[1..];
    v.into_iter().zip(v_lag).map(|(a, b)| a / b - 1.0).collect()
}

pub fn calculate_rolling_mean(x: &Vec<f64>, window: usize) -> Vec<f64> {
    let mut roll_mean = Vec::new();

    for i in 0..x.len() {
        if i < window {
            roll_mean.push(mean(&x[..=i]));
        } else {
            roll_mean.push(mean(&x[i - window + 1..=i]));
        }
    }

    roll_mean
}

pub fn calculate_rolling_std(x: &Vec<f64>, window: usize, ddof: Option<f64>) -> Vec<f64> {
    const DDOF: f64 = 1.0;
    let mut roll_std = Vec::new();

    for i in 0..x.len() {
        if i < window {
            roll_std.push(standard_deviation(&x[..=i], DDOF));
        } else {
            roll_std.push(standard_deviation(&x[i - window + 1..=i], DDOF));
        }
    }

    roll_std
}

pub fn cumulative_sum(x: &Vec<f64>) -> Vec<f64> {
    let mut cumul = x.clone();
    let mut last = 0.0;
    for i in 0..cumul.len() {
        last += cumul[i];
        cumul[i] = last;
    }
    cumul
}

pub fn array_max(x: &Vec<f64>) -> f64 {
    x.iter().fold(x[0], |acc, &x| acc.max(x))
}
pub fn array_min(x: &Vec<f64>) -> f64 {
    x.iter().fold(x[0], |acc, &x| acc.min(x))
}

pub fn calculate_ols_coefficients<T: na::RealField + Copy>(
    x_data: Vec<Vec<T>>,
    y_data: Vec<T>,
) -> Result<Vec<T>, &'static str> {
    let x_mat = na::DMatrix::from_fn(x_data.len(), x_data[0].len(), |i, j| x_data[i][j]);
    let y_vec = na::DVector::from_vec(y_data);

    let results = lstsq::lstsq(&x_mat, &y_vec, na::convert(1e-18f64)).unwrap();
    println!("{:#?}", results.solution[0]);
    println!("{:#?}", results.solution[1]);
    let decomp = na::SVD::new(x_mat.clone(), true, true);

    match decomp.solve(&y_vec, na::convert(1e-18f64)) {
        Ok(mat) => Ok(mat.data.into()),
        Err(error) => Err(error),
    }
}
