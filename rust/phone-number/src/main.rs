// A place for experiments with parser implementation

#[macro_use]
extern crate nom;

use std::iter;

// Digits
named!(x<char>, one_of!("0123456789"));
named!(n<char>, one_of!("23456789"));

// Separator
named!(sep, eat_separator!(" -."));

// Common components
named!(country_code, preceded!(opt!(tag!("+")), tag!("1")));

named!(
    nxx<String>,
    do_parse!(
    head_digit: n >>
    remaining_digits: count!(x, 2) >>
    (
        iter::once(&head_digit)
            .chain(remaining_digits.iter())
            .collect()
    )
)
);

named!(
    xxxx<String>,
    map!(count!(x, 4), |chars| chars.iter().collect())
);

// Put everything together
named!(
    phone_number<String>,
    do_parse!(
    opt!(country_code) >>
    sep >>
    area_code: alt!(
        nxx |
        delimited!(tag!("("), nxx, tag!(")"))
    ) >>
    sep >>
    exchange_code: nxx >>
    sep >>
    subscriber_num: xxxx >>
    eof!() >>
    (format!("{}|{}|{}",
        area_code,
        exchange_code,
        subscriber_num
    ))
)
);

fn main() {
    let test = "1  526  255  4555";
    println!("{:?}", phone_number(test.trim().as_bytes()));
}
