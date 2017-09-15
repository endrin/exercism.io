use std::ascii::AsciiExt;
use std::collections::BTreeMap;

pub fn transform(
    from: &BTreeMap<i32, Vec<char>>,
) -> BTreeMap<char, i32> {
    from.iter()
        .flat_map(|(score, chars)| {
            chars.iter().map(
                move |c| (c.to_ascii_lowercase(), *score),
            )
        })
        .collect()
}
