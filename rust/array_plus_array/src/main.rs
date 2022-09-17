fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    let mut sum_all: i32 = 0;
    for x in xs {
        sum_all += x;
    }
    for y in ys {
        sum_all += y;
    }
    sum_all
}
