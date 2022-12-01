use std::cmp::{max, min};

fn low(lhs: usize) -> usize {
    max(0, (lhs as i32) - 1) as usize
}

fn sum_adjacent_cells(cells: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut sum = 0;
    let base = max(0, low(i));
    let last = min(i + 2, cells.len());
    for row in &cells[base..last] {
        let base = max(0, low(j));
        let last = min(j + 2, row.len());
        for col in &row[base..last] {
            sum += if *col == '*' { 1 } else { 0 };
        }
    }
    sum
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let cells: Vec<Vec<char>> = minefield
        .iter()
        .map(|row| row.chars().collect())
        .collect();
    minefield.iter().enumerate().map(|(i, s)| {
        s.char_indices()
            .map(|(j, c)|
                match (c, sum_adjacent_cells(&cells, i, j)) {
                    ('*', _) => c,
                    (' ', 0) => c,
                    (_, total) => total.to_string().chars().next().unwrap(),
                },
            )
            .collect()
    }).collect()
}
