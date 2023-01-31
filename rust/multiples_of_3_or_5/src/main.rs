fn solution(num: i32) -> i32 {
    let mut sum = 0;
    for n in 0..num {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}
