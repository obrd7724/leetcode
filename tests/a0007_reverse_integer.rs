const TEST_LIST: [(i32, i32); 6] = [
    (123, 321),
    (-123, -321),
    (120, 21),
    (0, 0),
    (1534236469, 0),
    (1534236461, 1646324351),
];

#[test]
pub fn reverse_integer_s1() {
    for x in &TEST_LIST {
        assert_eq!(leet_code::reverse_integer(x.0), x.1);
    }
}
