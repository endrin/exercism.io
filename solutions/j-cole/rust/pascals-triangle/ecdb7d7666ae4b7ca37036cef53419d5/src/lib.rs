pub struct PascalsTriangle {
    num_rows: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { num_rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result = Vec::new();
        for _ in 0..self.num_rows {
            let row = PascalsTriangle::next_row(result.last().cloned());
            result.push(row);
        }
        result
    }

    fn next_row(current_row: Option<Vec<u32>>) -> Vec<u32> {
        match current_row {
            None => return vec![1],
            Some(r) => {
                return (0..1)
                    .chain(r)
                    .chain(0..1)
                    .collect::<Vec<u32>>()
                    .windows(2)
                    .map(|x| x.iter().sum())
                    .collect()
            }
        };
    }
}