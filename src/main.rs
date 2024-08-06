use crate::polynomial::Polynomial;

mod polynomial;
mod exponential;

fn main() {
    let coefficients: Vec<f64> =vec![1f64, 2f64];
    let poly: Polynomial = Polynomial::new(coefficients);
    let new_poly: Polynomial = poly.antiderivative();
    for i in 0..=new_poly.degree {
        println!("coef {}", new_poly.coefficients[i as usize]);
    }
    // let value: f64 = poly.int(0f64, 10.5f64);
    // println!("{}", value)
}
