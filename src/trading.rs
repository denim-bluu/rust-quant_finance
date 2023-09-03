use std::ops::Add;

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

    pub fn price_matrix(
        &self,
    ) -> na::Matrix<f64, na::Dyn, na::Dyn, na::VecStorage<f64, na::Dyn, na::Dyn>> {
        na::DMatrix::from_vec(self.x.len(), 2, [&self.x[..], &self.y[..]].concat())
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

    pub fn get_weights(
        &self,
        lookback: usize,
    ) -> na::Matrix<f64, na::Dyn, na::Dyn, na::VecStorage<f64, na::Dyn, na::Dyn>> {
        let hedge_ratios: Vec<f64> = self
            .calculate_hedge_ratio(lookback)
            .iter_mut()
            .map(|x| *x * -1.0 as f64)
            .collect();
        na::DMatrix::from_vec(hedge_ratios.len(), 1, hedge_ratios).insert_column(1, 1.0)
    }

    pub fn get_weighted_portfolio(&self, lookback: usize) -> Vec<f64> {
        let weights = self.get_weights(lookback);
        let prices: na::Matrix<f64, na::Dyn, na::Dyn, na::VecStorage<f64, na::Dyn, na::Dyn>> =
            self.price_matrix().remove_rows(0, lookback);
        weights
            .component_mul(&prices)
            .column_sum()
            .as_slice()
            .to_vec()
    }

    pub fn bollinger_band_mean_reversion_strategy(
        &self,
        entry_z_score: f64,
        exit_z_score: f64,
        lookback: usize,
    ) -> Vec<f64> {
        let portfolio = self.get_weighted_portfolio(lookback);
        let roll_mean = na::DVector::from_vec(math::calculate_rolling_mean(&portfolio, lookback));
        let roll_std =
            na::DVector::from_vec(math::calculate_rolling_std(&portfolio, lookback, Some(1.0)));
        let portf = na::DVector::from_vec(portfolio.to_owned());
        let roll_z = (portf - roll_mean).component_div(&roll_std);
        let long_entry: Vec<bool> = roll_z.iter().map(|x| *x < -entry_z_score).collect();
        let long_exit: Vec<bool> = roll_z.iter().map(|x| *x > -exit_z_score).collect();
        let short_entry: Vec<bool> = roll_z.iter().map(|x| *x > entry_z_score).collect();
        let short_exit: Vec<bool> = roll_z.iter().map(|x| *x < entry_z_score).collect();

        let n_unit_long: Vec<f64> = long_entry
            .iter()
            .map(|x| match x {
                true => 1.0,
                false => 0.0,
            })
            .collect();
        let n_unit_short: Vec<f64> = short_entry
            .iter()
            .map(|x| match x {
                true => -1.0,
                false => 0.0,
            })
            .collect();

        let n_units: Vec<f64> = n_unit_long
            .iter()
            .zip(n_unit_short.iter())
            .map(|(l, s)| l + s)
            .collect();

        let n_units =
            na::DMatrix::from_vec(n_units.len(), 2, [&n_units[..], &n_units[..]].concat());

        let weights = self.get_weights(lookback);
        let ret_x =
            na::DVector::from_vec(math::calculate_daily_returns(&self.x)).remove_rows(0, lookback);
        let ret_y =
            na::DVector::from_vec(math::calculate_daily_returns(&self.y)).remove_rows(0, lookback);

        let ret_mat = na::DMatrix::from_vec(
            ret_x.len(),
            2,
            [ret_x.as_slice(), ret_y.as_slice()].concat(),
        );

        let positions = n_units.component_mul(&weights).remove_rows(ret_y.len(), 1);
        let pnl = positions.component_mul(&ret_mat).column_sum();

        let ret = pnl.component_div(&positions.abs().column_sum());
        let apr = ret
            .add_scalar(1.0)
            .iter()
            .filter(|x| !x.is_nan())
            .product::<f64>()
            .powf(252.0 / ret.shape().0 as f64)
            - 1.0;

        println!("APR: {:#?}", apr);
        ret.as_slice().to_vec()
    }
}
