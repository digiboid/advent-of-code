use std::fs;

fn beam_sides(board: &mut Vec<Vec<char>>, rn: usize, cn: usize) {
    dbg!("matched beam sides");
    if board[rn - 1][cn] == '|' {
        dbg!("entered beam sides");
        board[rn][cn - 1] = '|';
        board[rn][cn + 1] = '|';
    }
}

fn beam_down(board: &mut Vec<Vec<char>>, rn: usize, cn: usize) {
    if rn + 1 < board.len() {
        if board[rn + 1][cn] == '.' {
            dbg!("entered beam down");

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

fn part_one(path: &str) {
    let contents = fs::read_to_string(path).expect("There's no file in here");
    let contents: Vec<&str> = contents.lines().collect();
    let mut board: Vec<Vec<char>> = contents.iter().map(|s| s.chars().collect()).collect();
    dbg!(&board[0][7]);
    // let mut reached_end = false;
    // HACK
    for i in 0..100 {
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
        // reached_end = board[board.len() - 1].iter().any(|c| *c == '|');
        // if reached_end == true {
        // break;
        // }
    }
    let split_count = count_splits(&board);
    let pretty_board: Vec<String> = board.iter().map(|l| l.iter().collect()).collect();
    dbg!(&pretty_board);
    dbg!(&split_count);
}
fn main() {
    println!("{:?}", part_one("test_input.txt"));
}
