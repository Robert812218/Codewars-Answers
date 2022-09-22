fn maps(values: &Vec<i32>) -> Vec<i32> {
    let mut new_vals: Vec<i32> = values.into_iter().map(|v| v * 2).collect();
    new_vals
}
