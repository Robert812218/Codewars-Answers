fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut new_arr = Vec::new();
    for (i, value) in arr.iter().enumerate() {
        if i % 2 == 0 {
            new_arr.push(*value);
        }
    }
    new_arr
}
