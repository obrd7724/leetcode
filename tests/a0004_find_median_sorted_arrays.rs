use leet_code;

#[test]
pub fn find_median_sorted_arrays_1() {
    assert_eq!(leet_code::find_median_sorted_arrays_s1(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10, 11]), 6_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s1(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]), 5.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s1(vec![1, 3], vec![2]), 2_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s1(vec![1, 2], vec![3, 4]), 2.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s1(vec![], vec![1]), 1_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s1(vec![0, 0], vec![0, 0]), 0_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s1(vec![2], vec![]), 2_f64);
}

#[test]
pub fn find_median_sorted_arrays_2() {
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![3,4],vec![]), 3.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10, 11]), 6_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]), 5.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![1, 3], vec![2]), 2_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![1, 2], vec![3, 4]), 2.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![], vec![1]), 1_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![0, 0], vec![0, 0]), 0_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s2(vec![2], vec![]), 2_f64);
}


#[test]
pub fn find_median_sorted_arrays_3() {
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![3,4],vec![]), 3.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10, 11]), 6_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]), 5.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![1, 3], vec![2]), 2_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![1, 2], vec![3, 4]), 2.5_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![], vec![1]), 1_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![0, 0], vec![0, 0]), 0_f64);
    assert_eq!(leet_code::find_median_sorted_arrays_s3(vec![2], vec![]), 2_f64);
}