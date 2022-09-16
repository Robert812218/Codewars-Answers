fn points(games: &[String]) -> u32 {
    let mut score = 0;
    for game in games {
        let eins = String::from(game).remove(0);
        let zwei = String::from(game).remove(2);
        let g_1 = eins.to_digit(16);
        let g_2 = zwei.to_digit(16);
        if g_1 > g_2 {
            score += 3;
        } else if g_2 == g_1 {
            score += 1;
        }
     }
     score
}
