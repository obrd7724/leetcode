const TEST_LIST: [(&'static str, i32, &'static str); 4] = [
    ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
    ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
    ("ABC", 2, "ACB"),
    ("A", 1, "A"),
];

#[test]
pub fn z_convert_s1() {
    for x in &TEST_LIST {
        assert_eq!(leet_code::z_convert_s1(x.0.to_string(), x.1), x.2);
    }
}

#[test]
pub fn z_convert_s2() {
    for x in &TEST_LIST {
        assert_eq!(leet_code::z_convert_s2(x.0.to_string(), x.1), x.2);
    }
}

#[test]
pub fn z_convert_s3() {
    for x in &TEST_LIST {
        assert_eq!(leet_code::z_convert_s3(x.0.to_string(), x.1), x.2);
    }
}