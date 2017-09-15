pub fn build_proverb(input: Vec<&str>) -> String {
    let first_words = input.iter();
    let second_words = input.iter().skip(1);
    let last_line = std::iter::once(match input.len() {
        0 => String::new(),
        1...2 => {
            String::from("And all for the want of a nail.")
        }
        _ => String::from(
            "And all for the want of a horseshoe nail.",
        ),
    });

    first_words
        .zip(second_words)
        .map(|(first, second)| {
            format!(
                "For want of a {} the {} was lost.",
                first,
                second
            )
        })
        .chain(last_line)
        .collect::<Vec<String>>()
        .join("\n")
}
