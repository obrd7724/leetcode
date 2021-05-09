use leet_code;

#[test]
fn length_of_longest_substring_1() {
    assert_eq!(leet_code::length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(leet_code::length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(leet_code::length_of_longest_substring(String::from("pwwkew")), 3);
    assert_eq!(leet_code::length_of_longest_substring(String::from("")), 0);
    // println!("{}", leet_code::length_of_longest_substring(String::from("")));
}