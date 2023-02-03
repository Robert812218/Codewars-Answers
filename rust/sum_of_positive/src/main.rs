fn positive_sum(slice: &[i32]) -> i32 {
    let mut sums: i32 = 0;
    for i in slice.iter() {
        if *i > 0 {
            sums += *i;
        }
    }
    sums
}
