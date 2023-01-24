fn bin_to_decimal(inp: &str) -> i32 {
    println!("inp: {}", inp);
    let mut dec: i32 = 0;
    for (i, ch) in inp.chars().enumerate() {
        println!("i: {}, ch: {}", i, ch);
        if ch == '1' {
           dec += 1i32 << (inp.len() - 1 - i);
        }
    }
    println!("dec: {}", dec);
    dec
}
