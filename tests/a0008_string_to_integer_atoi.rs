const TEST_LIST: [(&str, i32); 7] = [
    ("42", 42),
    ("   -42", -42),
    ("4193 with words", 4193),
    ("words and 987", 0),
    ("-91283472332", -2147483648),
    ("91283472332", 2147483647),
    ("9223372036854775808", 2147483647),
];

#[test]
pub fn string_to_integer_atoi_s1() {
    for x in &TEST_LIST {
        assert_eq!(leet_code::string_to_integer_atoi_s1(x.0.to_string()), x.1);
    }
}

#[test]
pub fn string_to_integer_atoi_s2() {
    for x in &TEST_LIST {
        assert_eq!(leet_code::string_to_integer_atoi_s2(x.0.to_string()), x.1);
    }
}