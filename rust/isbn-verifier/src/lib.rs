// Well, I've already started using nom,
// so there's no reason to stop

#[macro_use]
extern crate nom;

use nom::digit;
use std::str::FromStr;
use std::num::ParseIntError;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    validate_isbn(isbn.trim())
        .to_full_result()
        .unwrap_or(false)
}

named!(normal_digit<&str, Result<u8, ParseIntError>>,
    map!(digit, FromStr::from_str)
);


named!(check_digit<&str, Result<u8, ParseIntError>>,
    alt!(value!(Ok(10u8), tag!("X")) | normal_digit)
);

named!(maybe_dash<&str, Option<&str>>, opt!(tag!("-")));

named!(
    validate_isbn<&str, bool>,
    do_parse!(
    group_code: normal_digit >>
    maybe_dash >>
    publisher_code: count!(normal_digit, 3) >>
    maybe_dash >>
    title_code: count!(normal_digit, 5) >>
    maybe_dash >>
    final_check: check_digit >>
    ({
        // let isbn: Vec<_> = group_code.iter()
        //     .chain(publisher_code.iter())
        //     .chain(title_code.iter())
        //     .chain(final_check.iter())
        //     .collect();
        let mut isbn: Vec<u8> = vec![];
        isbn.extend(group_code);
        isbn.extend(publisher_code.iter().cloned().flat_map(Result::ok));
        isbn.extend(title_code.iter().cloned().flat_map(Result::ok));
        isbn.extend(final_check);

        println!("{:?}", isbn);

        isbn.iter().zip((1..11).rev()).map(|(&digit, pos)| digit * pos).map(usize::from).sum::<usize>() == 0
    })
)
);
