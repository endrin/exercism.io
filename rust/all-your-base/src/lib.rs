
pub fn convert(
    number: &[u32],
    from_base: u32,
    to_base: u32,
) -> Result<Vec<u32>, ()> {
    validate(number, from_base, to_base)
        .map(|_| conv_from_base(number, from_base))
        .map(|dec_number| conv_to_base(dec_number, to_base))
}

fn validate(
    number: &[u32],
    from_base: u32,
    to_base: u32,
) -> Result<(), ()> {
    if from_base < 2 || to_base < 2 ||
        number.iter().any(|d| d >= &from_base)
    {
        Err(())
    } else {
        Ok(())
    }
}

fn conv_from_base(digits: &[u32], base: u32) -> u32 {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(p, d)| d * base.pow(p as u32))
        .sum()
}

fn conv_to_base(number: u32, base: u32) -> Vec<u32> {
    let mut num = number;
    (0u32..)
        .map(|n| base.pow(n))
        .take_while(|n| n <= &number)
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .fold(vec![], |mut digits, &pos| {
            digits.push(num / pos);
            num = num % pos;
            digits
        })
}

#[cfg(test)]
mod tests;
