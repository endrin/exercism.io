pub fn lsp(series: &str, length: usize) -> Result<u32, ()> {
    if length == 0 {
        return Ok(1);
    }

    if series.chars().filter(|c| !c.is_digit(10)).count() >
        0
    {
        return Err(());
    }

    series
        .as_bytes()
        .windows(length)
        .map(|seq| {
            seq.iter()
                .filter_map(|c| (*c as char).to_digit(10))
                .product()
        })
        .max()
        .ok_or(())
}
