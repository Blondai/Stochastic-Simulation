// Traits
pub struct Exponential {
    pub(crate) factor: f64,
    pub(crate) exponent: f64,
    pub(crate) constant: f64
}

// Init
impl Exponential {
    pub fn new(factor: f64, exponent: f64, constant: f64) -> Self {
        return Self { factor, exponent, constant }
    }
}

// Evaluation Stuff
impl Exponential {
    pub fn evaluate(&self, input_value: f64) -> f64 {
        let exponent: f64 = self.exponent * input_value;
        let output_value: f64 = self.factor * exponent.exp() + self.constant;
        return output_value
    }

    pub fn eval(&self, input_value: f64) -> f64 {
        let output_value: f64 = self.evaluate(input_value);
        return output_value
    }
}

// Differentiation stuff
impl Exponential {
    pub fn differentiate(&self, number: u32) -> Exponential {
        let new_factor: f64 = self.factor * self.exponent.powi(number as i32);
        let new_exponent: f64 = self.exponent;
        let new_constant: f64 = 0f64;
        let new_exponential: Exponential = Exponential::new(new_factor,
                                                            new_exponent,
                                                            new_constant);
        return new_exponential
    }

    pub fn diff(&self, number: u32) -> Exponential {
        let new_polynomial: Exponential = self.differentiate(number);
        return new_polynomial
    }
}

// Integration stuff
impl Exponential {
    pub fn antiderivative(&self) -> Exponential {
        if self.constant != 0f64 {
            unimplemented!()
        }

        let new_factor: f64 = self.factor / self.exponent;
        let new_exponent: f64 = self.exponent;
        let new_constant: f64 = 0f64;
        let new_exponential: Exponential = Exponential::new(new_factor,
                                                            new_exponent,
                                                            new_constant);
        return new_exponential
    }

    pub fn integrate(&self, lower_bound: f64, upper_bound: f64) -> f64 {
        let new_polynomial: Exponential = self.antiderivative();
        let value: f64 = new_polynomial.eval(upper_bound) - new_polynomial.eval(lower_bound);
        return value
    }

    pub fn int(&self, lower_bound: f64, upper_bound: f64) -> f64 {
        let value: f64 = self.integrate(lower_bound, upper_bound);
        return value
    }
}