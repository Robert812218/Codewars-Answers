fn to_csv_text(array: &[Vec<i8>]) -> String {
    let mut big_str = String::from("");
    for arr in array {
        let mut sub_str = String::from("");
        for a in arr {
            sub_str = format!("{}{}{}", sub_str, ",", a);
        }
        sub_str.remove(0);
        sub_str = format!("{}{}", sub_str, "\n");
        big_str = format!("{}{}", big_str, sub_str);
    }
    big_str.pop();
    big_str
}
