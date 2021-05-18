use leet_code;

#[test]
pub fn longest_palindrome_1() {
    assert_eq!(leet_code::longest_palindrome_s1(String::from("aaa")), String::from("aaa"));
    assert_eq!(leet_code::longest_palindrome_s1(String::from("ababc")), String::from("aba"));
    assert_eq!(leet_code::longest_palindrome_s1(String::from("aaaa")), String::from("aaaa"));
    assert_eq!(leet_code::longest_palindrome_s1(String::from("babad")), String::from("bab"));
    assert_eq!(leet_code::longest_palindrome_s1(String::from("cbbd")), String::from("bb"));
    assert_eq!(leet_code::longest_palindrome_s1(String::from("a")), String::from("a"));
    assert_eq!(leet_code::longest_palindrome_s1(String::from("ac")), String::from("a"));
}

#[test]
pub fn longest_palindrome_2() {
    assert_eq!(leet_code::longest_palindrome_s2(String::from("aaa")), String::from("aaa"));
    assert_eq!(leet_code::longest_palindrome_s2(String::from("ababc")), String::from("aba"));
    assert_eq!(leet_code::longest_palindrome_s2(String::from("aaaa")), String::from("aaaa"));
    assert_eq!(leet_code::longest_palindrome_s2(String::from("babad")), String::from("bab"));
    assert_eq!(leet_code::longest_palindrome_s2(String::from("cbbd")), String::from("bb"));
    assert_eq!(leet_code::longest_palindrome_s2(String::from("a")), String::from("a"));
    assert_eq!(leet_code::longest_palindrome_s2(String::from("ac")), String::from("a"));
    assert_eq!(leet_code::longest_palindrome_s2(String::from("bb")), String::from("bb"));
}

#[test]
pub fn longest_palindrome_s3() {
    assert_eq!(leet_code::longest_palindrome_s3(String::from("abcdcbababcd")), String::from("dcbababcd"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("aaa")), String::from("aaa"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("ababc")), String::from("aba"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("aaaa")), String::from("aaaa"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("babad")), String::from("bab"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("cbbd")), String::from("bb"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("a")), String::from("a"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("ac")), String::from("a"));
    assert_eq!(leet_code::longest_palindrome_s3(String::from("bb")), String::from("bb"));
}

