use std::fmt;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = Some(Box::new(ListNode::new(0)));

    let mut a1 = l1;
    let mut a2 = l2;
    let mut n1 = 0;
    let mut n2 = 0;
    let mut j1 = 0;
    let mut result_opt = result.as_mut().unwrap();

    while a1.is_some() || a2.is_some() || j1 != 0 {
        if a1.is_none() && a2.is_none() {
            if j1 > 0 {
                result_opt.next = Some(Box::new(ListNode::new(j1)));
            }
            return result.unwrap().next;
        }
        result_opt.next = Some(Box::new(ListNode::new(0)));
        result_opt = result_opt.next.as_mut().unwrap();
        if a1.is_some() {
            let x = a1.unwrap();
            n1 = x.val;
            a1 = x.next;
        }
        if a2.is_some() {
            let x1 = a2.unwrap();
            n2 = x1.val;
            a2 = x1.next;
        }
        let sum = n1 + n2 + j1;
        if sum > 9 {
            j1 = 1;
            result_opt.val = sum % 10;
        } else {
            j1 = 0;
            result_opt.val = sum;
        }
        n1 = 0;
        n2 = 0;
    }
    result.unwrap().next
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn new_vec(nums: Vec<i32>) -> Self {
        if nums.len() >= 2 {
            let mut p1 = Some(Box::new(ListNode::new(nums[1])));
            let mut a1 = p1.as_mut().unwrap();
            let mut i = 2;
            while i < nums.len() {
                a1.next = Some(Box::new(ListNode::new(nums[i])));
                a1 = a1.next.as_mut().unwrap();
                i = i + 1;
            }
            ListNode {
                next: p1,
                val: nums[0],
            }
        } else {
            ListNode {
                next: None,
                val: nums[0],
            }
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut r = Vec::new();
        r.push(*&self.val);
        let mut result = &self.next;
        while result.is_some() {
            let x = result.as_ref().unwrap();
            r.push(*&x.val);
            result = &x.next;
        }
        r
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_vec())
    }
}