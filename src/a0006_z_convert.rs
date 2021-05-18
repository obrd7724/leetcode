pub fn z_convert_s1(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let columns = (s.len() as i32 / (num_rows * 2 - 2)) * (num_rows - 1) + s.len() as i32 % (num_rows * 2 - 2) % num_rows + 1;
    let mut dp = vec![vec![' ' as u8; columns as usize]; num_rows as usize];
    let mut column = 0;
    let mut pos: i32 = 0;
    let mut f = 0;
    let mut c_row = 0;
    loop {
        if pos == s.len() as i32 {
            break;
        }
        if f == 0 {
            dp[c_row as usize][column as usize] = s.as_bytes()[pos as usize];
            pos += 1;
            if c_row == num_rows - 1 {
                f = if num_rows == 2 {
                    c_row = 0;
                    0
                } else { 1 };
                column += 1;
                continue;
            }
            c_row += 1;
        } else {
            let c = (pos + 1) % (num_rows * 2 - 2);
            if c == 0 {
                dp[1][column as usize] = s.as_bytes()[pos as usize];
                f = 0;
                c_row = 0;
            } else {
                let row = num_rows - 1 - (c % num_rows);
                dp[row as usize][column as usize] = s.as_bytes()[pos as usize];
            }
            column += 1;
            pos += 1;
        }
    }
    let mut r = String::new();
    for row in dp {
        for c in row {
            if c == ' ' as u8 {
                continue;
            }
            r.push(c as char);
        }
    }
    r
}


pub fn z_convert_s2(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut r_vec = vec![String::new(); num_rows as usize];

    let mut f: i32 = -1;
    let mut row = 0;
    for c in s.as_bytes() {
        r_vec[row].push((*c) as char);
        f = if row == 0 || row == (num_rows - 1) as usize { -f } else { f };
        row = (row as i32 + f) as usize;
    }
    r_vec.join("")
}


pub fn z_convert_s3(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let len = s.len() as i32;
    let mut r = String::new();
    let cycle = num_rows * 2 - 2;

    for i in 0..num_rows {
        let mut j = 0;

        while j + i < len {
            r.push(s.as_bytes()[(j + i) as usize] as char);
            if i != 0 && i != num_rows - 1 && cycle - i + j  < len {
                r.push(s.as_bytes()[(cycle - i + j ) as usize] as char);
            }
            j += cycle;
        }
    }
    r
}