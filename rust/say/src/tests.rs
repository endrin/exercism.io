use super::*;

// Chunking tests

#[test]
fn chunk_large_number() {
    assert_eq!(chunked(1_234_567_890), &[890, 567, 234, 1]);
}

#[test]
fn chunk_thousand() {
    assert_eq!(chunked(1000), &[0, 1]);
}

#[test]
fn chunk_eight_hundred_and_ten_thousand() {
    assert_eq!(chunked(810_000), &[0, 810]);
}

// Annotation tests

#[test]
fn annotate_one_million() {
    assert_eq!(
        annotated(1_000_000),
        &[(1, String::from(" million"))]
    );
}

#[test]
fn annotate_1002000() {
    assert_eq!(
        annotated(1_002_000),
        &[
            (1, String::from(" million")),
            (2, String::from(" thousand"))
        ]
    );
}

#[test]
fn annotate_one_million_two() {
    assert_eq!(
        annotated(1_000_002),
        &[(1, String::from(" million")), (2, String::new())]
    );
}

#[test]
fn annotate_everything() {
    assert_eq!(
        annotated(9_223_372_036_854_775_807),
        &[
            (9, String::from(" quintillion")),
            (223, String::from(" quadrillion")),
            (372, String::from(" trillion")),
            (36, String::from(" billion")),
            (854, String::from(" million")),
            (775, String::from(" thousand")),
            (807, String::new())
        ]
    );
}
