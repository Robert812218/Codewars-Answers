fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let mut a_sum: i32 = 1;
    let mut b_sum: i32 = 1;
    a.iter().for_each(|i| a_sum *= i);
    b.iter().for_each(|j| b_sum *= j);
    let mut diff: i32 = a_sum - b_sum;
    if diff < 0 {
        diff *= -1;
    }
    diff
}
