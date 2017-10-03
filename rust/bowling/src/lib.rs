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
    // fn into_score(&self) -> (u32, u32) {
    //     match *self {
    //         Frame::New => (0, 0),
    //         Frame::Incomplete(r) => (r, 0),
    //         Frame::Normal(first, second) |
    //         Frame::Spare(first, second) => (first, second),
    //         Frame::Strike => (10, 0),
    //     }
    // }

    fn sum(&self) -> u32 {
        match *self {
            Frame::New => 0,
            Frame::Incomplete(r) => r,
            Frame::Normal(first, second) |
            Frame::Spare(first, second) => first + second,
            Frame::Strike => 10,
        }
    }

    fn rolls(&self) -> Box<Iterator<Item = u32>> {
        match *self {
            Frame::New => Box::new(iter::once(0)),
            Frame::Incomplete(r) => Box::new(iter::once(r)),
            Frame::Normal(first, second) |
            Frame::Spare(first, second) => Box::new(
                iter::once(first).chain(iter::once(second)),
            ),
            Frame::Strike => Box::new(iter::once(10)),
        }
    }
}

#[derive(PartialEq)]
enum Game {
    InProgress,
    BonusOneRoll,
    BonusTwoRolls,
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
        self.state == Game::Finished
    }

    fn save(&mut self, f: Frame) {
        match self.state {
            Game::InProgress => {
                &self.frames.push(f);
                if self.frames.len() == 10 {
                    match self.frames.last().unwrap() {
                        &Frame::Spare(_, _) => {
                            self.state = Game::BonusOneRoll
                        }
                        &Frame::Strike => {
                            self.state = Game::BonusTwoRolls
                        }
                        _ => self.state = Game::Finished,
                    }
                }
            }
            Game::BonusOneRoll => {
                &self.bonus.push(f);
                self.state = Game::Finished;
            }
            Game::BonusTwoRolls => {
                self.state = if let Frame::Strike = f {
                    Game::BonusOneRoll
                } else {
                    Game::Finished
                };
                &self.bonus.push(f);
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
            Frame::Incomplete(_)
                if self.state == Game::BonusOneRoll =>
            {
                self.save(frame_result)
            }
            _ => self.keep(frame_result),
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u32, &'static str> {
        if !self.finished() {
            return Err("Game not finished");
        }

        let game_score = self.frames
            .iter()
            .chain([Frame::New, Frame::New].iter())
            .tuple_windows()
            .fold(0, |score, (current, next, after_next)| {
                let rolls =
                    next.rolls().chain(after_next.rolls());

                let frame_score = match current {
                    &Frame::Strike => {
                        10u32 + &rolls.take(2).sum()
                    }
                    &Frame::Spare(_, _) => {
                        10u32 + &rolls.take(1).sum()
                    }
                    _ => current.sum(),
                };

                score + frame_score
            })
            + &self.bonus.iter().map(Frame::sum).sum();

        // Strike in the frame before last
        // actually doubles bonus o_O
        match self.frames.get(8).unwrap() {
            &Frame::Strike => Ok(
                game_score
                    + self.bonus
                        .get(0)
                        .unwrap_or(&Frame::New)
                        .rolls()
                        .next()
                        .unwrap(),
            ),
            _ => Ok(game_score),
        }
    }
}
