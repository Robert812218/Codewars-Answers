fn invert(values: &[i32]) -> Vec<i32> {
     let new_vals: Vec<i32> = values.into_iter().map(|v| v * -1).collect();
     new_vals
}
