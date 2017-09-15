pub fn hamming_distance(
    x: &str,
    y: &str,
) -> Result<usize, &'static str> {
    if x.len() != y.len() {
        Err("Length mismatch")
    } else {
        Ok(
            x.chars()
                .zip(y.chars())
                .filter(|&(xc, yc)| xc != yc)
                .count(),
        )
    }
}