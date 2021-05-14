pub fn find_median_sorted_arrays_s1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let k = (nums1.len() + nums2.len() + 1) / 2;

    let m = nums1.len();
    let n = nums2.len();
    if (m + n) % 2 == 1 {
        let _k = (nums1.len() + nums2.len() + 1) / 2;
        return find_index(&nums1, &nums2, k) as f64;
    }
    let k = (nums1.len() + nums2.len() + 1) / 2;
    let k1 = k + 1;
    return (find_index(&nums1, &nums2, k) + find_index(&nums1, &nums2, k1)) as f64 / 2_f64;
}

fn find_index(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k: usize) -> i32 {
    let m = nums1.len();
    let n = nums2.len();
    let mut m_offset = 0;
    let mut n_offset = 0;

    loop {
        if m_offset == m {
            return nums2[n_offset + k - 1];
        }
        if n_offset == n {
            return nums1[m_offset + k - 1];
        }
        if k == 1 {
            return nums1[m_offset].min(nums2[n_offset]);
        }

        let half = k / 2;
        let m_index = m.min(m_offset + half) - 1;
        let n_index = n.min(n_offset + half) - 1;
        if nums1[m_index] <= nums2[n_index] {
            k -= m_index - m_offset + 1;
            m_offset = m_index + 1;
        } else {
            k -= n_index - n_offset + 1;
            n_offset = n_index + 1;
        }
    }
}


pub fn find_median_sorted_arrays_s2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    let mut k = (nums1.len() + nums2.len() + 1) / 2;
    let mut m_offset = 0;
    let mut n_offset = 0;

    let k1;

    let is_one = (n + m) % 2 == 1;
    loop {
        if m_offset == m {
            k1 = nums2[n_offset + k - 1];
            n_offset = n_offset + k;
            break;
        }
        if n_offset == n {
            k1 = nums1[m_offset + k - 1];
            m_offset = m_offset + k;
            break;
        }
        if k == 1 {
            if nums1[m_offset] <= nums2[n_offset] {
                k1 = nums1[m_offset];
                m_offset = m_offset + k;
            } else {
                k1 = nums2[n_offset];
                n_offset = n_offset + k;
            }
            break;
        }

        let half = k / 2;
        let m_index = m.min(m_offset + half) - 1;
        let n_index = n.min(n_offset + half) - 1;
        if nums1[m_index] <= nums2[n_index] {
            k -= m_index - m_offset + 1;
            m_offset = m_index + 1;
        } else {
            k -= n_index - n_offset + 1;
            n_offset = n_index + 1;
        }
    }
    if is_one {
        return k1 as f64;
    }
    let k2;
    if m_offset == m {
        k2 = nums2[n_offset];
    } else if n_offset == n {
        k2 = nums1[m_offset];
    } else {
        k2 = nums1[m_offset].min(nums2[n_offset]);
    }
    return (k1 + k2) as f64 / 2_f64;
}

pub fn find_median_sorted_arrays_s3(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays_s3(nums2, nums1);
    }
    let m = nums1.len();
    let n = nums2.len();

    let mut left = 0;
    let mut right = m;

    let mut v1 = 0;
    let mut v2 = 0;
    while left <= right {
        let i = (right + left + 1) / 2;
        let j = (m + n + 1) / 2 - i;

        let l1v = if i == 0 { i32::MIN } else { nums1[i - 1] };
        let l2v = if j == 0 { i32::MIN } else { nums2[j - 1] };

        let r1v = if i == m { i32::MAX } else { nums1[i] };
        let r2v = if j == n { i32::MAX } else { nums2[j] };

        if l1v <= r2v {
            v1 = l1v.max(l2v);
            v2 = r1v.min(r2v);
            left = i + 1;
        } else {
            right = i - 1;
        }
    }


    return if (m + n) % 2 == 1 { v1 as f64 } else { (v1 + v2) as f64 / 2.0 };
}