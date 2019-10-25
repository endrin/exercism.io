pub fn is_armstrong_number(num: u32) -> bool {
    let digits = ((num as f64).log10() + 1.).floor() as u32;
    (0..digits)
        .map(|i| (num / 10u32.pow(i) % 10).pow(digits))
        .sum::<u32>() == num
}