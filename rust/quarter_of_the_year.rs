// Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.

// For example: month 2 (February), is part of the first quarter; month 6 (June), is part of the second quarter; and month 11 (November), is part of the fourth quarter.


fn quarter_of(month: u8) -> u8 {
    let mut this_month: u8 = 0;
    let mut inp_month: u8 = month;
    
    match inp_month {
        1 | 2 | 3 => this_month += 1,
        4 | 5 | 6 => this_month += 2,
        7 | 8 | 9 => this_month += 3,
        10 | 11 | 12 => this_month += 4,
        _ => println!("False input"),
    }
    
    this_month
}
