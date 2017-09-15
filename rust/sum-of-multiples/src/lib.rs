use std::collections::HashSet;

pub fn sum_of_multiples(
    max: usize,
    ms: &Vec<usize>,
) -> usize {
    ms.iter()
        .flat_map(|m| {
            Multiples::of(*m).take_while(|n| n < &max)
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .sum::<usize>()
}


struct Multiples {
    factor: usize,
    pos: usize,
}

impl Multiples {
    fn of(factor: usize) -> Multiples {
        Multiples { factor, pos: 0 }
    }
}

impl Iterator for Multiples {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        Some(self.pos * self.factor)
    }
}