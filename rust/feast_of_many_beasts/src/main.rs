fn feast(beast: &str, dish: &str) -> bool {
    let beast_chars: Vec<_> = beast.chars().collect();
    let beast_len = beast.len() - 1;
    let beast_start = beast_chars[0];
    let beast_end = beast_chars[beast_len];
    
    let dish_chars: Vec<_> = dish.chars().collect();
    let dish_len = dish.len() - 1;
    let dish_start = dish_chars[0];
    let dish_end = dish_chars[dish_len];
    if beast_start == dish_start && beast_end == dish_end {
        true
    } else {
        false
    }
}
