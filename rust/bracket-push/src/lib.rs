pub struct Brackets(String);

impl Brackets {
    pub fn from(input: &str) -> Self {
        Brackets(
            input
                .chars()
                .filter(|&c| "({[]})".contains(c))
                .collect(),
        )
    }

    pub fn are_balanced(&self) -> bool {
        let &Brackets(ref brs) = self;
        let mut stack: Vec<char> = vec![];

        for br in brs.chars() {
            match br {
                '[' | '{' | '(' => stack.push(br),
                ']' | '}' | ')' => if stack.pop()
                    != Some(Brackets::pair(br))
                {
                    return false;
                },
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }

    fn pair(br: char) -> char {
        match br {
            ']' => '[',
            '}' => '{',
            ')' => '(',
            _ => unreachable!(),
        }
    }
}
