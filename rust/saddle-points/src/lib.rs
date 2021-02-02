use std::collections::BTreeSet;

pub fn find_saddle_points(
    input: &[Vec<u64>],
) -> Vec<(usize, usize)> {
    let dims = check_dimensions(input);
    if dims.is_none() {
        return vec![];
    }

    let (width, height) = dims.unwrap();

    let max_rows: BTreeSet<(usize, usize)> = (0..height)
        .flat_map(|r| {
            let max_val = (0..width)
                .map(|c| input[r][c])
                .max()
                .unwrap();

            (0..width)
                .filter(move |&c| input[r][c] == max_val)
                .map(move |c| (r, c))
        })
        .collect();

    let min_cols: BTreeSet<(usize, usize)> = (0..width)
        .flat_map(|c| {
            let min_val = (0..height)
                .map(|r| input[r][c])
                .min()
                .unwrap();

            (0..height)
                .filter(move |&r| input[r][c] == min_val)
                .map(move |r| (r, c))
        })
        .collect();

    max_rows
        .intersection(&min_cols)
        .into_iter()
        .cloned()
        .collect()
}

fn check_dimensions(
    mtx: &[Vec<u64>],
) -> Option<(usize, usize)> {
    mtx.first().and_then(|row| {
        let width = row.len();
        if width == 0 {
            None
        } else {
            Some((width, mtx.len()))
        }
    })
}
