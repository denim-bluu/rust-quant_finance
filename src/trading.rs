use crate::math;
use polyfit_rs::polyfit_rs;
use nalgebra as na;

#[derive(Debug, Clone)]
pub struct AssetPairs {
    pub asset1: Vec<f64>,
    pub asset2: Vec<f64>,
}

impl AssetPairs {
    pub fn new(asset1: Vec<f64>, asset2: Vec<f64>) -> Result<Self, &'static str> {
        if asset1.len() != asset2.len() {
            Err("Assets must have the same length")
        } else {
            Ok(AssetPairs { asset1, asset2 })
        }
    }
    pub fn simple_linear_mean_reversion_strategy(&self, lookback: usize) -> Vec<f64> {
        let mut hedge_ratio: Vec<f64> = Vec::with_capacity(self.asset1.len() - lookback);
        let x = na::Matrix::from_vec_generic(na::Dyn(self.asset1.len()), na::Const::<1>, self.asset1.clone());
        let x_matrix: Vec<Vec<f64>> = self.asset1.iter().map(|&x| vec![1.0, x]).collect();
        let y_vec = self.asset2.clone();

        // for i in lookback..self.asset1.len() {
        //     let x = x_matrix.clone()[(i - lookback)..i].to_vec();
        //     let y = y_vec.clone()[(i - lookback)..i].to_vec();
        //     let coef = math::calculate_ols_coefficients(x, y).unwrap()[1];
        //     hedge_ratio.push(coef);
        // }
        // let x = x_matrix.clone()[(20 - lookback)..20].to_vec();
        let xx = y_vec.clone()[(20 - lookback)..20].to_vec();
        let y = y_vec.clone()[(20 - lookback)..20].to_vec();
        // let coef = math::calculate_ols_coefficients(x, y).unwrap();
        // let coef = polyfit_rs::polyfit(&x, &y, 1).unwrap();
        println!("{:#?}", x);
        println!("{:#?}", y);
        return hedge_ratio;
    }
}
