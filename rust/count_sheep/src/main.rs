fn count_sheep(n: u32) -> String {
    let mut sentence = String::new();  
    for i in 0..n {
        sentence.push_str(&format!("{} sheep...", i + 1))
    }
    println!("{}", sentence);
    sentence
}
