const TEST_LIST: [(i32, bool); 4] = [
    (121, true),
    (-121, false),
    (11, true),
    (0, true),
];


#[test]
pub fn palindrome_number() {
    for x in &TEST_LIST {
        assert_eq!(leet_code::palindrome_number(x.0), x.1);
    }
}