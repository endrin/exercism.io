pub fn encode(input: &str) -> String {
    let order = {
        let mut cs = input.chars().collect::<Vec<_>>();
        cs.dedup();
        cs
    };

    let mut remaining_input = input;

    order
        .iter()
        .map(|c| match remaining_input.find(|i| i != *c) {
            Some(idx) => {
                let chunk = &remaining_input[..idx];
                remaining_input = &remaining_input[idx..];
                chunk.len()
            }
            None => remaining_input.len(),
        })
        .map(|l| if l > 1 {
            l.to_string()
        } else {
            String::new()
        })
        .zip(order.iter())
        .map(|(n, c)| format!("{}{}", n, c))
        .collect()
}

pub fn decode(input: &str) -> String {
    input
        .split(|c: char| !c.is_digit(10))
        .map(|n_str| n_str.parse::<usize>().unwrap_or(1))
        .zip(input.matches(|c: char| !c.is_digit(10)))
        .map(|(num, c)| c.repeat(num))
        .collect()
}
