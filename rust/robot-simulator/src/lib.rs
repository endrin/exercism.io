// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

pub struct Robot {
    x: isize,
    y: isize,
    d: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: match self.d {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: match self.d {
                North => West,
                East => North,
                South => East,
                West => South,
            },
        }
    }

    fn move_adjustments(&self) -> (isize, isize) {
        match self.d {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }

    pub fn advance(self) -> Self {
        let (adj_x, adj_y) = self.move_adjustments();
        Robot {
            x: (self.x + adj_x),
            y: (self.y + adj_y),
            d: self.d,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(
            self,
            |robot, directive| match directive {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                c @ _ => panic!(
                    "The only allowed instructions are \
                     'L', 'R', or 'A'. '{}' is not allowed",
                    c
                ),
            },
        )
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
