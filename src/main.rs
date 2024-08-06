use crate::polynomial::Polynomial;

mod polynomial;

fn main() {
    let coefficients: Vec<f64> =vec![1f64];
    let poly: Polynomial = Polynomial::new(coefficients);
    let poly_d: Polynomial = poly.single_diff();
    for i in 0..=poly_d.degree {
        println!("coef {}", poly_d.coefficients[i as usize]);
    }


}
