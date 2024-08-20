use crate::distribution_function::DistributionFunction;

pub struct DensityFunction {
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    length: usize,
}

// New
impl DensityFunction {
    pub fn new(x_values: Vec<f64>, y_values: Vec<f64>) -> DensityFunction {
        // Change them inplace
        Self::_test_sorted(&x_values, &y_values);
        Self::_test_nonnegativity(&y_values);
        assert_eq!(x_values.len(), y_values.len(),
                   "Lengths are not the same {} != {}.", x_values.len(), y_values.len());
        let length: usize = x_values.len();
        DensityFunction {
            x_values,
            y_values,
            length,
        }
    }
}

// Tests
impl DensityFunction {
    fn _test_sorted(x_values: &Vec<f64>, y_values: &Vec<f64>) {
        let mut is_sorted: bool = true;
        for index in 0..(x_values.len() - 1) {
            if x_values[index] >= x_values[index + 1] {
                is_sorted = false;
                break;
            }
        }
        assert!(is_sorted, "The x-values are not sorted.")
    }

    fn _test_nonnegativity(y_values: &Vec<f64>) {
        const ZERO: f64 = 0f64;
        let mut is_nonnegative: bool = true;
        for value in y_values {
            if value < &ZERO {
                is_nonnegative = false;
                break;
            }
        }
        assert!(is_nonnegative, "The y-values are not all positive.")
    }
}

// Integration
impl DensityFunction {
    fn integrate(&self) -> (Vec<f64>, Vec<f64>) {
        let x_values: Vec<f64> = self.x_values.clone();
        let mut y_values: Vec<f64> = vec![0f64];
        for index in 0..self.length - 1 {
            let new_integral: f64 = 1f64 / 2f64 * (-self.x_values[index] * self.y_values[index]
                - self.x_values[index] * self.y_values[index + 1]
                + self.x_values[index + 1] * self.y_values[index]
                + self.x_values[index + 1] * self.y_values[index + 1]);
            y_values.push(new_integral + y_values[y_values.len() - 1])
        }
        (x_values, y_values)
    }
}

// Distribution Function
impl DensityFunction {
    pub fn distribution(&self) -> DistributionFunction {
        let pair: (Vec<f64>, Vec<f64>) = self.integrate();
        let x_values: Vec<f64> = pair.0;
        let y_values: Vec<f64> = pair.1;
        DistributionFunction::new(x_values, y_values)
    }
}

// Random Number Generation
impl DensityFunction {
    pub fn gen(&self) -> f64 {
        self.distribution().gen()
    }

    pub fn gens(&self, number: usize) -> Vec<f64> {
        self.distribution().gens(number)
    }
}

// Export
impl DensityFunction {
    pub fn export(self, file_name: &str, number: usize) {
        self.distribution().export(file_name, number)
    }
}
