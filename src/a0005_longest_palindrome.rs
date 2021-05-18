pub fn longest_palindrome_s1(q: String) -> String {
    let s = q.as_bytes();
    let t_len = s.len();
    if t_len < 2 {
        return q;
    }
    let mut dp = vec![vec![true; t_len]; t_len];
    let mut max = 1;
    let mut left = 0;
    let mut right = 0;

    for len in 2..t_len + 1 {
        for i in 0..t_len {
            let j = len + i - 1;
            if j >= t_len {
                break;
            }
            if s[i] == s[j] {
                if j - i < 3 {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = dp[i + 1][j - 1];
                }
            } else {
                dp[i][j] = false;
            }
            if dp[i][j] && j - i + 1 > max {
                left = i;
                right = j;
                max = j - i + 1;
            }
        }
    }
    String::from(&q.as_str()[left..right + 1])
}


pub fn longest_palindrome_s2(q: String) -> String {
    let mut max = 1;
    let mut left = 0;
    let mut right = 1;
    for i in 0..q.len() {
        let tp1 = expand_around_center(&q, i as i32, i as i32);
        let tp2 = expand_around_center(&q, i as i32, i as i32 + 1);
        let mut tp = tp2;
        if tp1.0 >= tp2.0 {
            tp = tp1;
        }
        if max < tp.0 {
            left = tp.1 as usize;
            right = tp.2;
            max = tp.0;
        }
    }
    String::from(&q.as_str()[left..right])
}

fn expand_around_center(s: &str, mut left: i32, mut right: i32) -> (usize, i32, usize) {
    while left >= 0 && (right as usize) < s.len() && s.as_bytes()[left as usize] == s.as_bytes()[right as usize] {
        left -= 1;
        right += 1;
    }
    ((right - left) as usize - 1, left + 1, right as usize)
}


pub fn longest_palindrome_s3(q: String) -> String {
    let mut s: Vec<u8> = Vec::new();
    s.push('#' as u8);
    for c in q.as_bytes() {
        s.push(*c);
        s.push('#' as u8);
    }
    let mut left = -1;
    let mut right = 1;
    let mut max = 0;

    let mut pos: i32 = -1;
    let mut pos_end: i32 = -1;

    let mut lens: Vec<i32> = Vec::new();
    for (i, _) in s.iter().enumerate() {
        let len;
        if i as i32 <= pos_end {
            let j = lens[(pos * 2) as usize - i].min(pos_end - i as i32);
            len = expand(&s, i as i32 - j, i as i32 + j);
        } else {
            len = expand(&s, i as i32, i as i32);
        }

        lens.push(len);

        if (i as i32 + len) > pos_end {
            pos = i as i32;
            pos_end = (i as i32 + len) as i32;
        }

        if max < (len as i32 * 2) + 1 {
            max = (len as i32 * 2) + 1;
            left = i as i32 - len;
            right = i as i32 + len;
        }
    }
    let mut result = String::new();
    for i in left..right {
        if s[i as usize] != '#' as u8 {
            result.push(s[i as usize] as char);
        }
    }
    result
}

fn expand(s: &Vec<u8>, mut left: i32, mut right: i32) -> i32 {
    while left >= 0 && (right as usize) < s.len() && s[left as usize] == s[right as usize] {
        left -= 1;
        right += 1;
    }
    (right - left - 2) / 2
}

