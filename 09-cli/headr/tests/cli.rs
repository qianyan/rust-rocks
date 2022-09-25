#[test]
fn test_parse_positive_int() {
    let res = headr::parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = headr::parse_positive_int("nonInteger");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "nonInteger".to_string());

    let res = headr::parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}