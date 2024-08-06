
pub struct Polynomial {
    pub(crate) coefficients: Vec<f64>,
    pub(crate) degree: u32,
}

// Init
impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let degree: u32 = (coefficients.len() as u32) - 1;
        return Self { coefficients, degree }
    }
}

// Evaluation Stuff
impl Polynomial {
    pub fn evaluate(&self, input_value: f64) -> f64 {
        let mut output_value: f64 = 0f64;
        let mut potences: f64 = 1f64;
        for coefficient in &self.coefficients {
            output_value += coefficient * potences;
            potences *= input_value;
        }
        return output_value
    }

    pub fn eval(&self, input_value: f64) -> f64 {
        return self.evaluate(input_value)
    }
}

// Differentiation stuff
impl Polynomial {
    pub fn single_diff(&self) -> Polynomial {
        let mut new_coefficients: Vec<f64> = Vec::new();
        if self.degree <= 1 {
            return Polynomial::new(vec![0f64])
        }

        for (i, &coefficient) in self.coefficients.iter().enumerate() {
            if i > 0 {
                let new_coefficient: f64 = coefficient * (i as f64);
                new_coefficients.push(new_coefficient)
            }
        }
        let new_polynomial: Polynomial = Polynomial::new(new_coefficients);
        return new_polynomial
    }

    pub fn differentiate(&self, number: u32) -> Polynomial {
        let coefficients: Vec<f64> = self.coefficients.clone();
        let mut new_polynomial: Polynomial = Polynomial::new(coefficients);
        for _ in 0..number {
            new_polynomial = new_polynomial.single_diff()
        }
        return new_polynomial
    }

    pub fn diff(&self, number: u32) -> Polynomial {
        let new_polynomial: Polynomial = self.differentiate(number);
        return new_polynomial
    }
}