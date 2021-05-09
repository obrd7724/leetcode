use std::collections::HashMap;
use std::vec::Vec;
pub struct Problems;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut scores = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        if let Some(&number) = scores.get(&(target - v)) {
            return vec![number as i32, i as i32];
        }
        scores.insert(*v, i);
    }
    vec![]
}

impl Problems {


}

