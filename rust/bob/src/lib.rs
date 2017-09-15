const ANSWERS: [(&'static Fn(&str) -> bool, &'static str);
    4] = [
    (&|s| s.trim().is_empty(), "Fine. Be that way!"),
    (
        &|s| {
            s.chars().any(|c| c.is_uppercase()) &&
                s.chars().all(|c| !c.is_lowercase())
        },
        "Whoa, chill out!",
    ),
    (&|s| s.trim().ends_with("?"), "Sure."),
    (&|_| true, "Whatever."),
];


pub fn reply(question: &str) -> &str {
    let &(_, response) = ANSWERS
        .iter()
        .find(|&&(condition_satisfied, _)| {
            condition_satisfied(question)
        })
        .unwrap();
    response
}
