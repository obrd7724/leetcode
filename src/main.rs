mod problems;

use problems::two_sum;
use crate::problems::two_sum::ListNode;

fn main() {
    let sum = two_sum::Problems::two_sum(vec![1, 7, 3, 5], 12);
    println!("{:?}", sum);

    println!("{}", two_sum::Problems::add_two_numbers(Some(Box::new(ListNode::new_vec(vec![9, 9, 9, 9, 9, 9, 9])))
                                                      , Some(Box::new(ListNode::new_vec(vec![9, 9, 9, 9])))).unwrap());



    println!("{}", two_sum::Problems::length_of_longest_substring(  String::from("abcabcbb")));
    println!("{}", two_sum::Problems::length_of_longest_substring(  String::from("bbbbb")));
    println!("{}", two_sum::Problems::length_of_longest_substring(  String::from("pwwkew")));
    println!("Hello, world!");
}
