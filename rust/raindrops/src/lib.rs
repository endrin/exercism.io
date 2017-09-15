static DROPS: [(u32, &str); 3] =
    [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let sounds = DROPS
        .iter()
        .filter_map(
            |&(divisor, sound)| if n % divisor == 0 {
                Some(sound)
            } else {
                None
            },
        )
        .collect::<String>();

    if sounds.is_empty() {
        n.to_string()
    } else {
        sounds
    }
}
