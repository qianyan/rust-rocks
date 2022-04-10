mod parse;
mod complex;

use complex::*;

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);

    println!("{:?}", c1 + c2);
}
