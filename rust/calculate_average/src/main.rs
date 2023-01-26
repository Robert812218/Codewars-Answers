fn find_average(slice: &[f64]) -> f64 {
    let mut total = 0.0;
    let mut count = 0;

    for num in slice {
        total += *num;
        count += 1;
    }

    if count == 0 {
        0.0
    } else {
        total / (count as f64)
    }
}
