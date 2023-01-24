fn two_sort(arr: &[&str]) -> String {
    let s = arr.iter().min().unwrap();
    s.chars().map(|c|c.to_string()).collect::<Vec<String>>().join("***")
}
