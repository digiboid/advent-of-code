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
    // dbg!(&pretty_board);
    split_count
}
fn main() {
    println!("Splits: {:?}", part_one("input.txt"));
}
