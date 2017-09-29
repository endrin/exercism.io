pub struct ChessPosition {
    row: i32,
    col: i32,
}

impl ChessPosition {
    pub fn new(row: i32, col: i32) -> Result<Self, ()> {
        if let (0...7, 0...7) = (row, col) {
            Ok(ChessPosition { row, col })
        } else {
            Err(())
        }
    }
}

pub struct Queen(i32, i32);

impl Queen {
    pub fn new(pos: ChessPosition) -> Self {
        let ChessPosition { row, col } = pos;
        Queen(row, col)
    }

    pub fn can_attack(&self, other: &Self) -> bool {
        let &Queen(self_row, self_col) = self;
        let &Queen(other_row, other_col) = other;

        self_row == other_row || self_col == other_col
            || (self_row - other_row).abs()
                == (self_col - other_col).abs()
    }
}
