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
    fn _get_index(values: &Vec<f64>, input_value: f64) -> usize {
        for index in 0..values.len() {
            if input_value <= values[index] {
                return index - 1;
            }
        }
        return values.len() - 1;
    }

    fn _evaluate(&self, input_value: f64) -> f64 {
        if input_value < self.x_values[0usize] {
            return 0f64;
        } else if input_value > self.x_values[self.length - 1usize] {
            return 1f64;
        }
        let index: usize = DistributionFunction::_get_index(&self.x_values, input_value);
        let numerator: f64 =
            self.y_values[index] * (self.x_values[index + 1] - input_value)
                + self.y_values[index + 1] * (input_value - self.x_values[index]);
        let denominator: f64 = self.x_values[index + 1] - self.x_values[index];
        numerator / denominator
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