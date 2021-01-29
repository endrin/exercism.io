use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    balanced = @{ SOI ~ BRACKETS ~ EOI }
    BRACKETS = {
        ( NOT_BRACKETS
        | "[" ~ BRACKETS ~ "]"
        | "{" ~ BRACKETS ~ "}"
        | "(" ~ BRACKETS ~ ")" )*
    }
    NOT_BRACKETS = { (!("[" | "]" | "{" | "}" | "(" | ")") ~ ANY)+ }
"#]
struct BalanceChecker;

pub fn brackets_are_balanced(input: &str) -> bool {
    BalanceChecker::parse(Rule::balanced, input).is_ok()
}
