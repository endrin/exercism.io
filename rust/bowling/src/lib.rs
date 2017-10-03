extern crate itertools;

use std::iter;
use itertools::Itertools;

enum Frame {
    New,
    Incomplete(u32),
    Normal(u32, u32),
    Spare(u32, u32),
    Strike,
}

impl Frame {
    fn into_score(&self) -> (u32, u32) {
        match *self {
            Frame::New => (0, 0),
            Frame::Incomplete(r) => (r, 0),
            Frame::Normal(first, second) |
            Frame::Spare(first, second) => (first, second),
            Frame::Strike => (10, 0),
        }
    }

    fn sum(&self) -> u32 {
        match *self {
            Frame::New => 0,
            Frame::Incomplete(r) => r,
            Frame::Normal(first, second) |
            Frame::Spare(first, second) => first + second,
            Frame::Strike => 10,
        }
    }
}

enum Game {
    InProgress,
    BonusRound(u32),
    Finished,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    bonus: Vec<Frame>,
    active_frame: Frame,
    state: Game,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frames = Vec::with_capacity(10);
        let bonus = Vec::with_capacity(2);
        BowlingGame {
            frames,
            bonus,
            active_frame: Frame::New,
            state: Game::InProgress,
        }
    }

    fn finished(&self) -> bool {
        match self.state {
            Game::Finished => true,
            _ => false,
        }
    }

    fn save(&mut self, f: Frame) {
        match self.state {
            Game::InProgress => {
                &self.frames.push(f);
                if self.frames.len() == 10 {
                    match self.frames.last().unwrap() {
                        &Frame::Spare(_, _) => {
                            self.state = Game::BonusRound(1)
                        }
                        &Frame::Strike => {
                            self.state = Game::BonusRound(2)
                        }
                        _ => self.state = Game::Finished,
                    }
                }
            }
            Game::BonusRound(n) => {
                &self.bonus.push(f);
                self.state = if n == 1 {
                    Game::Finished
                } else {
                    Game::BonusRound(1)
                }
            }
            _ => unreachable!(),
        }
        self.active_frame = Frame::New;
    }

    fn keep(&mut self, f: Frame) {
        self.active_frame = f;
    }

    fn process_roll(
        &self,
        roll: u32,
    ) -> Result<Frame, &'static str> {
        match self.active_frame {
            Frame::New if roll < 10 => {
                Ok(Frame::Incomplete(roll))
            }
            Frame::New if roll == 10 => Ok(Frame::Strike),

            Frame::Incomplete(prev_roll)
                if prev_roll + roll < 10 =>
            {
                Ok(Frame::Normal(prev_roll, roll))
            }
            Frame::Incomplete(prev_roll)
                if prev_roll + roll == 10 =>
            {
                Ok(Frame::Spare(prev_roll, roll))
            }

            _ => Err("Rolled invalid number of pins"),
        }
    }

    pub fn roll(
        &mut self,
        roll: u32,
    ) -> Result<(), &'static str> {
        if let Game::Finished = self.state {
            return Err("Game is finished");
        }

        let frame_result = self.process_roll(roll)?;

        match frame_result {
            Frame::Normal(_, _) |
            Frame::Spare(_, _) |
            Frame::Strike => self.save(frame_result),
            _ => self.keep(frame_result),
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u32, &'static str> {
        if !self.finished() {
            return Err("Game not finished");
        }

        let game_score = iter::once(&Frame::New)
            .chain(self.frames.iter())
            .tuple_windows()
            .fold(0, |score, (prev_frame, cur_frame)| {
                let (fst, snd) = cur_frame.into_score();

                let frame_score = match prev_frame {
                    &Frame::Spare(_, _) => fst * 2 + snd,
                    &Frame::Strike => fst * 2 + snd * 2,
                    _ => fst + snd,
                };

                score + frame_score
            })
            + &self.bonus.iter().map(Frame::sum).sum();

        Ok(game_score)
    }

    // fn get_score(
    //     frames: &Vec<u32>,
    //     bonus: &Vec<u32>,
    // ) -> u32 {
    //     frames
    //         .chunks(2)
    //         .fold(
    //             (false, 0),
    //             |(had_spare, total_score), frame| {
    //                 let mut dyn_frame = frame.iter();
    //                 let first_roll = dyn_frame.next().map(
    //                     |scr| if had_spare {
    //                         scr * 2
    //                     } else {
    //                         *scr
    //                     },
    //                 );
    //                 let second_roll = dyn_frame.next();

    //                 let frame_score = first_roll
    //                     .and(second_roll)
    //                     .map(|_| {
    //                         first_roll.unwrap()
    //                             + second_roll.unwrap()
    //                     })
    //                     .unwrap_or(first_roll.unwrap());

    //                 let had_spare =
    //                     frame.iter().sum::<u32>() == 10;

    //                 (had_spare, total_score + frame_score)
    //             },
    //         )
    //         .1 + &bonus.iter().sum()
    // }
}
