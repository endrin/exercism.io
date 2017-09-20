const NORMAL: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const DOUBLED: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
static TRANSFORMATIONS: [[u32; 10]; 2] = [NORMAL, DOUBLED];

pub fn is_valid(input: &str) -> bool {
    cleaned(input)
        .map(|digits| {
            let transformations =
                TRANSFORMATIONS.iter().cycle();

            digits
                .into_iter()
                .zip(transformations)
                .map(|(c, transform)| transform[c])
                .sum::<u32>() % 10 == 0
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
