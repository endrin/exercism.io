pub struct PascalsTriangle {
    all_rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return Self { all_rows: vec![] };
        }

        let total_rows = row_count as usize;

        let mut pt = Self {
            all_rows: Vec::with_capacity(total_rows),
        };

        pt.all_rows.push(vec![1]);

        for c in 2..total_rows + 1 {
            let mut next_row = Vec::with_capacity(c);
            next_row.push(1);
            for nums in pt.all_rows[c - 2].windows(2) {
                next_row.push(nums[0] + nums[1]);
            }
            next_row.push(1);
            pt.all_rows.push(next_row);
        }

        pt
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.all_rows.to_vec()
    }
}
