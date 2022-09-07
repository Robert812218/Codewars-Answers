fn divisors(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    
    let mut count: u32 = 0;
    
    for i in 1..n {
       if n > 0 {
            if n % i == 0 {
                count += 1;
            }   
        }
        
    }
    
    count + 1
}
