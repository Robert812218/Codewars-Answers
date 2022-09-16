fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() <= 1 {
        return None;
    }
    let mut nums_sum = 0;
    let mut new_arr = arr.to_vec();
    new_arr.sort_unstable();
    new_arr = new_arr.into_iter().rev().collect();

    
    for n in 1..arr.len() {
        nums_sum += new_arr[n - 1] - new_arr[n]
    }

    Some(nums_sum)
}
