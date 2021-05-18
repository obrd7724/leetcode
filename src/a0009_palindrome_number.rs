pub fn palindrome_number(mut x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut last = 0;

    while x > last {
        last = last * 10 + x % 10;
        x /= 10;
    }

    return last == x || last / 10 == x;
}