mod parse;
mod complex;
mod dyn_dispatch;
mod static_dispatch;
mod quiz;

use complex::*;
use dyn_dispatch::run as format;
use static_dispatch::run as cat;
use quiz::run as quiz;

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);

    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + 5.0);
    println!("{:?}", c1 + c2);

    format();
    cat();
    quiz();
}
