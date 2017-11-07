// Hey, nom is fun!

#[macro_use]
extern crate nom;

use nom::digit;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(_isbn: &str) -> bool {
    unimplemented!();
}

// named!(
//     normal_digit<u8>,
//     map!(verify!(take!(1), is_digit), |c| {
//         char::from(c[0]).to_digit(10)
//     })
// );
named!(
    normal_digit<u8>,
    map!(call!(digit), |c: &[u8]| {
        char::from(c[0]).to_digit(10)
    })
);


named!(
    check_digit<u8>,
    alt!(value!(10u8, tag!("X")) | normal_digit)
);
named!(maybe_dash<Option<&[u8]>>, opt!(tag!("-")));

named!(
    validate_isbn<bool>,
    do_parse!(
    group_code: normal_digit >>
    maybe_dash >>
    publisher_code: count!(normal_digit, 3) >>
    maybe_dash >>
    title_code: count!(normal_digit, 5) >>
    maybe_dash >>
    final_check: check_digit >>
    ({
        let mut isbn: Vec<u8> = vec![];
        isbn.push(group_code);
        isbn.extend(publisher_code);
        isbn.extend(title_code);
        isbn.push(final_check);

        isbn.iter().zip((1..11).rev()).map(|(&digit, pos)| digit * pos).sum() == 0
    })
)
);
