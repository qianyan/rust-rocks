use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self {real, imagine}
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}
// 为&Complex 实现 Add，如此可以实现 &c1 + &c2
impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

#[test]
fn override_add_for_complex() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    assert_eq!(&c1 + &c2, Complex::new(3.0, 4.0));
}

