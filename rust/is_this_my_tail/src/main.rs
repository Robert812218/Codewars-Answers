fn correct_tail(body: &str, tail: char) -> bool {
    let bytes = body.as_bytes();
    let len = bytes.len();
    let last_char = body.chars().nth(len - 1);
    last_char == Some(tail)
}
