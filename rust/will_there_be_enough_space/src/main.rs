fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    if wait > (cap - on) {
        wait - (cap - on)
    } else {
        0
    }
}
