#![feature(bool_to_option)]

use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    top_three: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        scores.iter().cloned().collect()
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.top_three.first().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_three.iter().cloned().collect()
    }
}

impl FromIterator<u32> for HighScores {
    fn from_iter<T: IntoIterator<Item = u32>>(
        iter: T,
    ) -> Self {
        let mut top_three = Vec::with_capacity(4);
        let scores = iter
            .into_iter()
            .map(|n| {
                let n_fits_top = top_three.len() < 3
                    || top_three.last().unwrap() < &n;

                if n_fits_top {
                    let pos = top_three
                        .iter()
                        .enumerate()
                        .find_map(|(pos, &sc)| {
                            (n >= sc).then_some(pos)
                        })
                        .unwrap_or(top_three.len());
                    top_three.insert(pos, n);
                    if top_three.len() == 4 {
                        top_three.pop();
                    }
                }

                n
            })
            .collect();

        Self { scores, top_three }
    }
}
