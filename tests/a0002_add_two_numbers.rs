use leet_code;
use leet_code::a0002_add_two_numbers::ListNode;

#[test]
fn add_two_numbers_1() {
    assert_eq!(
        leet_code::add_two_numbers(
            Some(Box::new(ListNode::new_vec(vec![9, 9, 9, 9, 9, 9, 9]))),
            Some(Box::new(ListNode::new_vec(vec![9, 9, 9, 9])))).unwrap().to_vec(),
        vec![8, 9, 9, 9, 0, 0, 0, 1]
    )
}