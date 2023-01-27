fn is_in_middle(seq: &str) -> bool {
    let mut mid: &str = seq;
    while mid.len() >= 5 {
        mid = &mid[1..mid.len()-1];
    }
    if mid.len() <= 4 && mid.contains("abc") {
        return true;
    }
    return false;
}
