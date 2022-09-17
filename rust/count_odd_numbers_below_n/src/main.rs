fn odd_count(n: u64) -> u64 {
    let mut new_n: u64 = n;
    if new_n < 1 {
        return 0;
    } 
    if new_n % 2 != 0 {
        new_n -= 1;
    }
    let outp = new_n / 2;
    outp
} 

