pub struct BowlingGame {
    frames: Vec<u32>,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frames = Vec::with_capacity(20);
        BowlingGame { frames }
    }

    pub fn roll(&mut self, roll: u32) -> Result<(), ()> {
        match roll {
            0...10 if self.score().is_err() => {
                if !self.frames.is_empty()
                    && self.frames.len() % 2 == 0
                    && self.frames
                        .iter()
                        .rev()
                        .take(2)
                        .sum::<u32>()
                        == 10
                {
                    &self.frames.push(roll * 2);
                } else {
                    &self.frames.push(roll);
                }
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn score(&self) -> Result<u32, ()> {
        if self.frames.len() < 20 {
            Err(())
        } else {
            Ok(BowlingGame::do_score(&self.frames))
        }
    }

    fn do_score(frames: &Vec<u32>) -> u32 {
        let mut deserves_double = false;
        frames
            .chunks(2)
            .map(|frame| if deserves_double {
                deserves_double = frame.iter().sum::<u32>() == 10;

                let mut dyn_frame = frame.iter();
                let first_roll =
                    dyn_frame.next().map(|scr| scr * 2);
                let second_roll = dyn_frame.next();
                first_roll
                    .and(second_roll)
                    .map(|_| {
                        first_roll.unwrap()
                            + second_roll.unwrap()
                    })
                    .unwrap_or(first_roll.unwrap())
            } else {
                deserves_double = frame.iter().sum::<u32>() == 10;

                let mut dyn_frame = frame.iter();
                let first_roll = dyn_frame.next();
                let second_roll = dyn_frame.next();
                first_roll
                    .and(second_roll)
                    .map(|_| {
                        first_roll.unwrap()
                            + second_roll.unwrap()
                    })
                    .unwrap_or(first_roll.unwrap().to_owned())
            })
            .sum()
    }
}
