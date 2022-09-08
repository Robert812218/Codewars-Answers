fn warn_the_sheep(queue: &[&str]) -> String {
    
    let mut booleroni: bool = false;
    let length = queue.len();
    let mut hodl: i32 = 0;
    println!("length: {}", length);

    let mut sentence = String::new();
    let mut wolf_position: i32 = 0;
    
    for i in queue.iter() {
        hodl += 1;
        println!("item: {} index: {}", i, hodl);
        if format!("{}", i) == "wolf" {
            wolf_position += hodl;
        }
    }
    println!("{}", wolf_position);
    if wolf_position ==  queue.len().try_into().unwrap() {
        sentence.push_str("Pls go away and stop eating my sheep");
    } else {
        println!("wolf: {} len: {}", wolf_position, length);
        let diff: i32 = length as i32 - wolf_position;
        println!("diff: {}", diff);
        sentence.push_str(&format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", diff));
    } 
    println!("{}", sentence);
    sentence
}
