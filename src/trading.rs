use crate::math;
use nalgebra as na;

#[derive(Debug, Clone)]
pub struct AssetPairs {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

impl AssetPairs {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Result<Self, &'static str> {
        if x.len() != y.len() {
            Err("Assets must have the same length")
        } else {
            Ok(AssetPairs { x, y })
        }
    }
    pub fn calculate_hedge_ratio(&self, lookback: usize) -> Vec<f64> {
        let mut hedge_ratio: Vec<f64> = Vec::with_capacity(self.x.len() - lookback);

        for i in lookback..self.x.len() {
            let x_vec = self.x[(i - lookback)..i].to_vec();
            let y_vec = self.y[(i - lookback)..i].to_vec();

            let x = na::DMatrix::from_vec(x_vec.len(), 1, x_vec);
            let x = x.insert_column(1, 1.0); // Add constant
            let y = na::DMatrix::from_vec(y_vec.len(), 1, y_vec.to_vec());
            let coef = math::calculate_ols_coefficients(x, y).unwrap()[0];
            hedge_ratio.push(coef);
        }
        return hedge_ratio;
    }
}
