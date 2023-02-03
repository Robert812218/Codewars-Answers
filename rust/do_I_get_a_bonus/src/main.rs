fn bonus_time(salary: u64, bonus: bool) -> String {
    if bonus {
        return format!("¥{}", salary * 10);
    } else {
        return format!("¥{}", salary);
    }
}
