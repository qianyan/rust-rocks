use std::str::FromStr;
use regex::Regex;

pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
        where
            Self: Sized;
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
// @see Default trait，绝大多数类型都实现了这个 trait，为数据结构提供缺省值。
//impl<T> Parse for T
//where 
//    T: FromStr + Default,
//{
//    fn parse(s: &str) -> Self {
//        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
//
//        let d = || Default::default();
//
//        if let Some(captures) = re.captures(s) {
//            captures
//                .get(0)
//                .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
//        } else {
//            d()
//        }
//    }
//}

impl<T> Parse for T
where 
    T: FromStr,
{
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();

        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| 
                    {
                        s.as_str()
                            .parse()
                            .map_err(|_err| "failed to parse captured string".to_string())
                    })
        } else {
            Err("failed parse string".to_string())
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), Ok(123));
    assert_eq!(
        u32::parse("123.45abcd"), 
        Err("failed to parse captured string".into()));
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}

fn main() {

}
