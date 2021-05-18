use std::collections::HashMap;

pub fn string_to_integer_atoi_s1(s: String) -> i32 {
    let mut sign = 0;
    let mut ans: i64 = 0;

    for c in s.as_bytes() {
        if ' ' as u8 == *c {
            if sign == 0 {
                continue;
            } else {
                break;
            }
        }
        if '+' as u8 == *c {
            if sign == 0 {
                sign = 1;
                continue;
            } else {
                break;
            }
        }
        if '-' as u8 == *c {
            if sign == 0 {
                sign = -1;
                continue;
            } else {
                break;
            }
        }
        if (*c as char).is_numeric() {
            if sign == 0 {
                sign = 1;
            }
            ans = ans * 10 + (c - '0' as u8) as i64;
            if sign == 1 && ans > i32::MAX as i64 {
                return i32::MAX;
            } else if sign == -1 && ans * sign < i32::MIN as i64 {
                return i32::MIN;
            }
            continue;
        }
        break;
    }
    (sign * ans) as i32
}

pub fn string_to_integer_atoi_s2(s: String) -> i32 {
    let table: HashMap<&str, [&str; 4]> = [("start", ["start", "signed", "is_number", "end"]),
        ("signed", ["end", "end", "is_number", "end"]),
        ("is_number", ["end", "end", "is_number", "end"]),
        ("end", ["end", "end", "end", "end"]),
    ].iter().cloned().collect();

    let mut state = "start";
    let mut sign = 1;
    let mut ans: i64 = 0;

    for c in s.as_bytes() {
        state = table[state][get_col(*c as char)];
        if state == "signed" {
            if '-' as u8 == *c { sign = -1; }
            continue;
        }
        if state == "is_number" {
            ans = ans * 10 + (c - '0' as u8) as i64;
            if sign == 1 && ans > i32::MAX as i64 {
                return i32::MAX;
            } else if sign == -1 && ans * sign < i32::MIN as i64 {
                return i32::MIN;
            }
        }
    }
    ans *= sign;
    ans as i32
}

fn get_col(c: char) -> usize {
    if ' ' == c { return 0; }
    if '+' == c || '-' == c { return 1; }
    if c.is_numeric() { return 2; }
    return 3;
}