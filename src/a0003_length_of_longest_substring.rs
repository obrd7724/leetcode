use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut map: HashMap<&char, Vec<usize>> = HashMap::new();
    let mut first_index = 0;
    let mut max = 0;
    let mut f = Vec::new();

    for (i, c) in chars.iter().enumerate() {
        match map.get_mut(c) {
            Some(idx) => {
                f.push((first_index, i as i32));
                let count = i - first_index;
                if max < count {
                    max = count;
                }
                if first_index == idx[idx.len() - 1] {
                    first_index = first_index + 1;
                } else if i - idx[idx.len() - 1] == 1 {
                    first_index = i;
                } else {
                    first_index = idx[idx.len() - 1];
                }
                idx.push(i);
            }
            None => {
                let mut idx = Vec::new();
                idx.push(i);
                map.insert(c, idx);
            }
        };
    }

    println!("{:?}", f);
    max as i32
}