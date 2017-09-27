pub struct Roman(u32);

pub use std::string::ToString;

impl Roman {
    const REGISTERS: [(
        u32,
        &'static str,
        &'static str,
        &'static str,
    ); 4] = [
        (1000, "M", "", ""),
        (100, "C", "D", "M"),
        (10, "X", "L", "C"),
        (1, "I", "V", "X"),
    ];

    pub fn from(n: u32) -> Self {
        assert!(n < 4000, "Entered number too big");
        Roman(n)
    }
}

impl ToString for Roman {
    fn to_string(&self) -> String {
        let &Roman(mut num) = self;

        Roman::REGISTERS.iter().fold(
            String::new(),
            |acc, &(lim, unit, five, ten)| if num >= lim {
                let digit = num / lim;
                num = num % lim;
                format!(
                    "{}{}",
                    acc,
                    parse(digit, unit, five, ten)
                )
            } else {
                acc
            },
        )
    }
}

fn parse(
    num: u32,
    unit: &str,
    five: &str,
    ten: &str,
) -> String {
    match num {
        1 => format!("{0}", unit),
        2 => format!("{0}{0}", unit),
        3 => format!("{0}{0}{0}", unit),
        4 => format!("{0}{1}", unit, five),
        5 => format!("{0}", five),
        6 => format!("{1}{0}", unit, five),
        7 => format!("{1}{0}{0}", unit, five),
        8 => format!("{1}{0}{0}{0}", unit, five),
        9 => format!("{0}{1}", unit, ten),
        _ => unreachable!(),
    }
}
