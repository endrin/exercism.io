enum Roll {
    NotPlayed,
    Normal(u32),
    Spare(u32),
    Strike,
}

use Roll::*;

impl Roll {
    fn score(&self) -> u32 {
        match *self {
            NotPlayed => 0,
            Normal(n) | Spare(n) => n,
            Strike => 10,
        }
    }
}

enum GameState {
    InProgress,
    BonusRound,
    Finished,
}

use GameState::*;

type Frame = (Roll, Roll);

pub struct BowlingGame {
    frames: Vec<Frame>,
    bonus: Vec<Frame>,
    current_frame: Frame,
    state: GameState,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frames = Vec::with_capacity(10);
        let bonus = Vec::with_capacity(2);
        BowlingGame {
            frames,
            bonus,
            current_frame: (NotPlayed, NotPlayed),
            state: InProgress,
        }
    }

    fn save(&mut self) {
        match self.state {
            InProgress => {
                &self.frames.push(self.current_frame);
            }
            BonusRound => {
                &self.bonus.push(self.current_frame);
            }
            Finished => unreachable!(),
        }
        self.current_frame = (NotPlayed, NotPlayed);
    }

    pub fn roll(&mut self, roll: u32) -> Result<(), ()> {
        match roll {
            0...10 if self.frames.len() < 20 => {
                &self.frames.push(roll);
                Ok(())
            }
            0...10 if self.bonus.len() < 2 => {
                &self.bonus.push(roll);
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn score(&self) -> Result<u32, ()> {
        if self.frames.len() < 20 {
            Err(())
        } else {
            Ok(BowlingGame::get_score(
                &self.frames,
                &self.bonus,
            ))
        }
    }

    fn get_score(
        frames: &Vec<u32>,
        bonus: &Vec<u32>,
    ) -> u32 {
        frames
            .chunks(2)
            .fold(
                (false, 0),
                |(had_spare, total_score), frame| {
                    let mut dyn_frame = frame.iter();
                    let first_roll = dyn_frame.next().map(
                        |scr| if had_spare {
                            scr * 2
                        } else {
                            *scr
                        },
                    );
                    let second_roll = dyn_frame.next();

                    let frame_score = first_roll
                        .and(second_roll)
                        .map(|_| {
                            first_roll.unwrap()
                                + second_roll.unwrap()
                        })
                        .unwrap_or(first_roll.unwrap());

                    let had_spare =
                        frame.iter().sum::<u32>() == 10;

                    (had_spare, total_score + frame_score)
                },
            )
            .1 + &bonus.iter().sum()
    }
}
