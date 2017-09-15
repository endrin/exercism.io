#![feature(inclusive_range_syntax)]

const MAX_BOTTLES: usize = 99;

pub fn verse(stock: usize) -> String {
    [top_line(stock), bottom_line(stock), String::new()]
        .join("\n")
}

pub fn sing(from: usize, to: usize) -> String {
    (to...from)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}


fn top_line(stock: usize) -> String {
    capitalized(
        format!("{0} on the wall, {0}.", how_many(stock)),
    )
}

fn bottom_line(stock: usize) -> String {
    format!(
        "{}, {} on the wall.",
        what_to_do(stock),
        how_many(
            stock.checked_sub(1).unwrap_or(MAX_BOTTLES)
        )
    )
}


fn how_many(stock: usize) -> String {
    match stock {
        0 => String::from("no more bottles of beer"),
        1 => format!("{} bottle of beer", stock),
        _ => format!("{} bottles of beer", stock),
    }
}

fn what_to_do(stock: usize) -> String {
    match stock {
        0 => String::from(
            "Go to the store and buy some more",
        ),
        1 => {
            String::from("Take it down and pass it around")
        }
        _ => {
            String::from("Take one down and pass it around")
        }
    }
}


fn capitalized(line: String) -> String {
    let mut chars = line.chars();
    chars
        .next()
        .into_iter()
        .flat_map(|c| c.to_uppercase())
        .chain(chars)
        .collect()
}
