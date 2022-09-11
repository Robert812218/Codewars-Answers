fn century(year: u32) -> u32 {
    let mut short_yr: u32 = year;
    if short_yr % 100 == 0 {
       return short_yr / 100; 
    } else {
       short_yr += 1;
       return (short_yr / 100) + 1;
    }
}

