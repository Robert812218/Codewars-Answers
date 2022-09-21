fn combat(health: f32, damage: f32) -> f32 {
    let remain: f32 = health - damage;
    if remain > 0.0 {
        remain
    } else {
        0.0
    }
}
