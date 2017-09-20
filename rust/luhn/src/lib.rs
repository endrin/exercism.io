static DOUBLED: [usize; 10] =
    [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

pub fn is_valid(input: &str) -> bool {
    cleaned(input)
        .map(|digits| {
            let double_these =
                [false, true].iter().cloned().cycle();

            digits
                .into_iter()
                .zip(double_these)
                .map(|(c, double_it)| if double_it {
                    DOUBLED[c]
                } else {
                    c
                })
                .sum::<usize>() % 10 == 0
        })
        .unwrap_or(false)
}

fn cleaned(input: &str) -> Option<Vec<usize>> {
    if input.chars().any(|c| !"0123546789 ".contains(c)) {
        return None;
    }

    let result: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as usize)
        .rev()
        .collect();

    if result.len() >= 2 {
        Some(result)
    } else {
        None
    }
}
