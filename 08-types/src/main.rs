use std::str::FromStr;
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

//impl Parse for u8 {
//    fn parse(s: &str) -> Self {
//        let re: Regex = Regex::new(r"^[0-9]+").unwrap();
//        if let Some(captures) = re.captures(s) {
//            captures
//                .get(0)
//                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
//        } else {
//            0
//        }
//    }
//}
//
//impl Parse for f64 {
//    fn parse(s: &str) -> Self {
//        let re: Regex = Regex::new(r"^[0-9]+\.[0-9]+").unwrap();
//        if let Some(captures) = re.captures(s) {
//            captures
//                .get(0)
//                .map_or(0.0, |s| s.as_str().parse().unwrap_or(0.0))
//        } else {
//            0.0
//        }
//    }
//}
//
impl<T> Parse for T
where 
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();

        let d = || Default::default();

        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    // 越界了自动变成0
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abcd"), 0);

    assert_eq!(f64::parse("123.45abcd"), 123.45);
}

fn main() {
    println!("result: {}", u8::parse("255 hello world"));
}
