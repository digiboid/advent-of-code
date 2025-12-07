use std::fs;

fn beam_sides(board: &mut Vec<Vec<char>>, rn: usize, cn: usize) {
    if board[rn - 1][cn] == '|' {
        board[rn][cn - 1] = '|';
        board[rn][cn + 1] = '|';
    }
}

fn beam_down(board: &mut Vec<Vec<char>>, rn: usize, cn: usize) {
    if rn + 1 < board.len() {
        if board[rn + 1][cn] == '.' {
            board[rn + 1][cn] = '|'
        }
    }
}

fn count_splits(board: &Vec<Vec<char>>) -> usize {
    let mut splits = 0;
    for rn in 0..board.len() {
        for cn in 0..board[0].len() {
            if board[rn][cn] == '^' {
                if board[rn - 1][cn] == '|' {
                    splits += 1;
                }
            }
        }
    }
    splits
}

fn part_one(path: &str) -> usize {
    let contents = fs::read_to_string(path).expect("There's no file in here");
    let contents: Vec<&str> = contents.lines().collect();
    let mut board: Vec<Vec<char>> = contents.iter().map(|s| s.chars().collect()).collect();
    let mut split_count = count_splits(&board);
    loop {
        split_count = count_splits(&board);
        for rn in 0..board.len() {
            for cn in 0..board[0].len() {
                match board[rn][cn] {
                    'S' => board[rn + 1][cn] = '|',
                    '^' => beam_sides(&mut board, rn, cn),
                    '|' => beam_down(&mut board, rn, cn),
                    _ => continue,
                }
            }
        }
        if split_count == count_splits(&board) {
            break;
        }
    }
    let pretty_board: Vec<String> = board.iter().map(|l| l.iter().collect()).collect();
    dbg!(&pretty_board);
    split_count
}

fn part_two(path: &str) -> usize {
    let contents = fs::read_to_string(path).expect("There's no file in here");
    let grid: Vec<&[u8]> = contents.lines().map(str::as_bytes).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let start = grid[0].iter().position(|&c| c == b'S').unwrap();
    let mut splits = vec![vec![false; cols]; rows];
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell == &b'^' {
                splits[row_idx][col_idx] = true;
            }
        }
    }
    let mut visited = vec![vec![0usize; cols]; rows];
    count_timelines(0, start, rows, &splits, &mut visited)
}

fn count_timelines(
    row: usize,
    col: usize,
    row_count: usize,
    splits: &Vec<Vec<bool>>,
    visited: &mut Vec<Vec<usize>>,
) -> usize {
    // skip visited cells
    if visited[row][col] != 0 {
        return visited[row][col];
    }

    if row == row_count - 1 {
        // mark cell as visited
        visited[row][col] = 1;
        return 1;
    }

    // propagate down if not at splitter
    if !splits[row][col] {
        // mark cell as visited
        let timelines = count_timelines(row + 1, col, row_count, splits, visited);
        visited[row][col] = timelines;
        return timelines;
    }

    // split
    let timelines = count_timelines(row, col + 1, row_count, splits, visited)
        + count_timelines(row, col - 1, row_count, splits, visited);
    visited[row][col] = timelines;
    timelines
}

fn main() {
    println!("Timelines: {:?}", part_two("input.txt"));
}
