fn multi_table(n: u64) -> String {
    let mut big_str = String::from("");
    for i in 1..11 {
        let mult = i * n;
        big_str.push_str(&format!("{} * {} = {}\n", i, n, mult));         
    }
    big_str.pop();
    big_str
}
