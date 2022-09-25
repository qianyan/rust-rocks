use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into())
    }
}