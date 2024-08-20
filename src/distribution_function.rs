use rand::Rng;
use rand::rngs::ThreadRng;

pub struct DistributionFunction {
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    length: usize,
}

// New
impl DistributionFunction {
    pub fn new(x_values: Vec<f64>, y_values: Vec<f64>) -> DistributionFunction {
        // Change them inplace
        Self::_test_sorted(&x_values, &y_values);
        Self::_test_monotony(&y_values);
        Self::_test_boundary(&y_values);
        assert_eq!(x_values.len(), y_values.len(),
                   "Lengths are not the same {} != {}.", x_values.len(), y_values.len());
        let length: usize = x_values.len();
        DistributionFunction {
            x_values,
            y_values,
            length,
        }
    }
}

// Tests
impl DistributionFunction {
    // TODO: Add corrections to this methods
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

    fn _test_monotony(y_values: &Vec<f64>) {
        let mut is_monoton: bool = true;
        for index in 0..(y_values.len() - 1) {
            if y_values[index] > y_values[index + 1] {
                is_monoton = false;
                break;
            }
        }
        assert!(is_monoton, "The y-values are not monotonly increasing.")
    }

    fn _test_boundary(y_values: &Vec<f64>) {
        const ACCURACY: f64 = DistributionFunction::_get_accuracy();
        let mut is_correctly_bounded: bool = true;
        if y_values[0] < 0f64 - ACCURACY || y_values[0] > 0f64 + ACCURACY {
            is_correctly_bounded = false
        }
        if y_values[y_values.len() - 1] < 1f64 - ACCURACY || y_values[y_values.len() - 1] > 1f64 + ACCURACY {
            is_correctly_bounded = false
        }
        // Add pretty formatstring
        assert!(is_correctly_bounded, "The y-values are not between 0 ± {} and 1 ± {}.", ACCURACY, ACCURACY)
    }
}

// Evaluation
impl DistributionFunction {
    fn _get_index(values: &Vec<f64>, input_value: f64) -> (usize, bool) {
        for index in 0..values.len() {
            if input_value == values[index] {
                return (index, true);
            }
            if input_value < values[index] {
                return (index - 1, false);
            }
        }
        return (values.len() - 1, false);
    }

    fn _evaluate(&self, input_value: f64) -> f64 {
        if input_value < self.x_values[0usize] {
            return 0f64;
        } else if input_value > self.x_values[self.length - 1usize] {
            return 1f64;
        }
        let tuple: (usize, bool) = DistributionFunction::_get_index(&self.x_values, input_value);
        let index: usize = tuple.0;
        let is_exact: bool = tuple.1;
        if is_exact {
            return self.y_values[index];
        } else {
            let numerator: f64 = self.y_values[index + 1] - self.y_values[index];
            let denominator: f64 = self.x_values[index + 1] - self.x_values[index];
            let factor: f64 = input_value - self.x_values[index];
            return numerator / denominator * factor + self.y_values[index];
        }
    }

    pub fn eval(self, input_value: f64) -> f64 {
        self._evaluate(input_value)
    }

    pub fn evals(self, input_values: Vec<f64>) -> Vec<f64> {
        let mut output_values: Vec<f64> = Vec::new();
        for input_value in input_values {
            output_values.push(self._evaluate(input_value))
        }
        output_values
    }
}

// Constants
impl DistributionFunction {
    const fn _get_accuracy() -> f64 {
        const ACCURACY: f64 = 0.01;
        ACCURACY
    }
}

// Solver
impl DistributionFunction {
    fn solve(&self, output_value: f64) -> f64 {
        DistributionFunction::_test_output_value(output_value);
        let tuple: (usize, bool) = DistributionFunction::_get_index(&self.y_values, output_value);
        let index: usize = tuple.0;
        let is_exact: bool = tuple.1;
        if is_exact {
            return self.x_values[index];
        } else {
            let numerator: f64 = self.x_values[index + 1] - self.x_values[index];
            let denominator: f64 = self.y_values[index + 1] - self.y_values[index];
            let factor: f64 = output_value - self.y_values[index];
            return numerator / denominator * factor + self.x_values[index];
        }
    }

    fn _test_output_value(output_value: f64) {
        const ACCURACY: f64 = DistributionFunction::_get_accuracy();
        let mut correct_output_value: bool = true;
        if output_value > 1f64 + ACCURACY {
            correct_output_value = false
        }
        if output_value < 0f64 - ACCURACY {
            correct_output_value = false
        }
        assert!(correct_output_value, "Value '{}' is not between 0 ± {} and 1 ± {}", output_value, ACCURACY, ACCURACY)
    }
}

// Uniform Distribution
impl DistributionFunction {
    fn _single_uniform() -> f64 {
        let mut rng: ThreadRng = rand::thread_rng();
        rng.gen()
    }
}

// Inverse Transform Sampling
impl DistributionFunction {
    fn _single_inverse_transform_sampling(&self) -> f64 {
        let uniform: f64 = DistributionFunction::_single_uniform();
        self.solve(uniform)
    }

    fn _inverse_transform_sampling(&self, number: u32) -> Vec<f64> {
        let mut vector: Vec<f64> = Vec::new();
        for _ in 0..number {
            vector.push(self._single_inverse_transform_sampling())
        }
        vector
    }
}

// Random Number Generation
impl DistributionFunction {
    pub fn gen(&self) -> f64 {
        self._single_inverse_transform_sampling()
    }

    pub fn gens(&self, number: u32) -> Vec<f64> {
        self._inverse_transform_sampling(number)
    }
}
