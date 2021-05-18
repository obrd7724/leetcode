// rev = rev * 10 +d
// i32::MAX 2147483647
// rev * 10 +d <= i32::MAX/10 * 10 +7 =>  (rev - i32::MAX/10) * 10  <= 7-d
// rev > i32::MAX/10 时 d >= 0  不成立
// rev = i32::MAX/10 时 d <= 7 成立
// rev < i32::MAX/10 时 d <= 9 成立
// 因 d => max = 2, 故 rev >  i32::MAX/10 等式不成立
pub fn reverse_integer(x: i32) -> i32 {
    let mut v = x;
    let mut rev = 0;
    while v != 0 {
        if rev > i32::MAX / 10 || rev < i32::MIN / 10 {
            return 0;
        }
        let d = v % 10;
        rev = rev * 10 + d;
        v /= 10;
    }
    rev
}