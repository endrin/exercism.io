
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let unit = "I";
        let five = "V";
        let ten = "X";
        let test_data = [
            (1, "I"),
            (2, "II"),
            (3, "III"),
            (4, "IV"),
            (5, "V"),
            (6, "VI"),
            (7, "VII"),
            (8, "VIII"),
            (9, "IX"),
        ];
        for &(input, output) in test_data.iter() {
            assert_eq!(
                parse(input, unit, five, ten),
                String::from(output),
                "Failed on {}",
                input
            );
        }
    }
}