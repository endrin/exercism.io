static ONES: [&str; 10] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

static TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static TENS: [&str; 10] = [
    "",
    "",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

static POWERS: [&str; 7] = [
    "",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
];

pub fn encode(num: u64) -> String {
    if num == 0 {
        zero()
    } else {
        encode_multiple_chunks(num)
    }
}

fn zero() -> String {
    String::from("zero")
}

fn encode_single_chunk(num: usize) -> Option<String> {
    match num {
        1...9 => Some(ones(num as usize)),
        10...19 => Some(teens(num as usize)),
        10...99 => Some(tens(num as usize)),
        100...999 => Some(hundreds(num as usize)),
        _ => None,
    }
}

fn encode_multiple_chunks(num: u64) -> String {
    annotated(num)
        .into_iter()
        .map(|(num, pwr)| {
            format!(
                "{}{}",
                encode_single_chunk(num).unwrap(),
                pwr
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn ones(n: usize) -> String {
    String::from(*ONES.get(n).unwrap_or(&""))
}

fn teens(n: usize) -> String {
    String::from(*TEENS.get(n - 10).unwrap_or(&""))
}

fn tens(n: usize) -> String {
    if n % 10 == 0 {
        String::from(*TENS.get(n / 10).unwrap_or(&""))
    } else {
        format!(
            "{}-{}",
            String::from(*TENS.get(n / 10).unwrap_or(&"")),
            ones(n % 10)
        )
    }
}

fn hundreds(n: usize) -> String {
    let h = format!("{} hundred", ONES[n / 100]);
    if n % 100 == 0 {
        h
    } else {
        format!(
            "{} {}",
            h,
            encode_single_chunk(n % 100).unwrap()
        )
    }
}

fn chunked(n: u64) -> Vec<usize> {
    // Picked this up using only error messages
    // from compiler.
    // I'm sure this whole sequence can be
    // somehow simplified, but don't want to
    // think about this now.

    // Also, as this function is used only to
    // attach annotations to the groups of thousands
    // so it would be better for it to return an iterator,
    // but I have no idea what to write as the return type
    // in this case.
    n.to_string()
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|t| {
            t.iter()
                .rev()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .concat()
                .parse::<usize>()
                .unwrap()
        })
        .collect()
}

fn annotated(n: u64) -> Vec<(usize, String)> {
    chunked(n)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, num)| if num == 0 {
            None
        } else {
            Some((num, String::from(POWERS[idx])))
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod tests;
