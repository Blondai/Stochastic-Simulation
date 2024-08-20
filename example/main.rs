use crate::distribution_function::DistributionFunction;

mod distribution_function;
mod normal_distribution;
use crate::normal_distribution::{NormalDistribution};

fn main() {
    let normal_distribution: NormalDistribution = NormalDistribution::new(0f64, 1f64);
    let tuple: (Vec<f64>, Vec<f64>) = normal_distribution.distribution_function(10000);
    let x_values: Vec<f64> = tuple.0;
    let y_values: Vec<f64> = tuple.1;
    let linear: DistributionFunction = DistributionFunction::new(x_values, y_values);
    linear.export("test.txt", 1_000_000)
}