pub struct NormalDistribution {
    mu: f64,
    sigma: f64,
}

// New
impl NormalDistribution {
    pub fn new(mu: f64, sigma: f64) -> NormalDistribution {
        NormalDistribution { mu, sigma }
    }
}

// Values of Density and Distribution
impl NormalDistribution {
    const SQRT_2: f64 = std::f64::consts::SQRT_2;

    fn density(&self, input_value: f64) -> f64 {
        const PI: f64 = std::f64::consts::PI;
        const E: f64 = std::f64::consts::E;
        1f64 / (2f64 * PI * self.sigma.powi(2)).sqrt() * E.powf(-(input_value - self.mu).powi(2) / (2f64 * self.sigma.powi(2)))
    }

    fn distribution(&self, input_value: f64) -> f64 {
        use statrs::function::erf::erf;
        0.5 * (1f64 + erf((input_value - self.mu) / (self.sigma * NormalDistribution::SQRT_2)))
    }
}

// Get Distribution Function and Density Function
impl NormalDistribution {
    pub fn distribution_function(&self, number: usize) -> (Vec<f64>, Vec<f64>) {
        let mut input_values: Vec<f64> = Vec::new();
        let mut output_values: Vec<f64> = Vec::new();

        let distance: f64 = (8f64 * self.sigma) / number as f64;

        let mut input_value: f64 = self.mu - 4f64 * self.sigma;
        for _ in 0..number {
            input_values.push(input_value);
            output_values.push(self.distribution(input_value));
            input_value += distance
        }

        (input_values, output_values)
    }

    pub fn density_function(&self, number: usize) -> (Vec<f64>, Vec<f64>) {
        let mut input_values: Vec<f64> = Vec::new();
        let mut output_values: Vec<f64> = Vec::new();

        let distance: f64 = (8f64 * self.sigma) / number as f64;

        let mut input_value: f64 = self.mu - 4f64 * self.sigma;
        for _ in 0..number {
            input_values.push(input_value);
            output_values.push(self.density(input_value));
            input_value += distance
        }

        (input_values, output_values)
    }
}
