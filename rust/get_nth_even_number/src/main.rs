fn nth_even(n: u32) -> u32 {
    if n == 1 {
        return 0;
    }
    (n * 2) - 2
}
