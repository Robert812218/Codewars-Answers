fn odd_count(n: u64) -> u64 {
    let mut num_count = 0;
    for i in 1..n {
        if i % 2 != 0 {
            num_count += 1;
        }
    }
    num_count
}
