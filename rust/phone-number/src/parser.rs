// phone::number("(223) 456-7890"),
// to_some_string("2234567890")

// named!(delim, opt!(ws!()));

named!(country_code, preceded!(opt!(tag!("+")), tag!("1")));

// named!(area_code);
// named!(exchange_code);

#[cfg(test)]
mod tests {
    use super::*;

    use nom::{IResult};

    #[test]
    fn just_country() {
        assert_eq!(country_code(&b"1"[..]), IResult::Done(&b""[..], "1"));
    }

    #[test]
    fn country_plus() {
        assert_eq!(country_code(&b"+1"[..]), IResult::Done(&b""[..], "1"));
    }

    #[test]
    fn not_a_country() {
        assert_eq!(country_code(&b"+2"[..]), IResult::Error(_));
    }
}