fn usdcny(usd: u16) -> String {
    let yu = usd as f32 * 6.75;
    format!("{:.2} Chinese Yuan", yu)
}
