use rand::prelude::*;
use std::{any, fs};

fn beam_sides(board: &mut Vec<Vec<char>>, rn: usize, cn: usize) {
    if board[rn - 1][cn] == '|' {
        board[rn][cn - 1] = '|';
        board[rn][cn + 1] = '|';
    }
}

fn beam_quantum(board: &mut Vec<Vec<char>>, rn: usize, cn: usize, split_left: &mut bool) {
    if board[rn - 1][cn] == '|' {
        let mut rng = rand::rng();
        if rng.random_bool(0.5) {
            board[rn][cn + 1] = '|';
        } else {
            board[rn][cn - 1] = '|';
            *split_left = true;
            // dbg!(&split_left);
        };
    }
}

fn beam_down(board: &mut Vec<Vec<char>>, rn: usize, cn: usize) {
    if rn + 1 < board.len() {
        if board[rn + 1][cn] == '.' {
            board[rn + 1][cn] = '|'
        }
    }
}

fn beam_down_alt(board: &mut Vec<Vec<char>>, rn: usize, cn: usize) {
    if rn < board.len() {
        if board[rn][cn] == '.' {
            board[rn][cn] = '|'
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
    let contents: Vec<&str> = contents.lines().collect();
    let timelines = 0;
    let mut split_left = false;
    let mut all_boards: Vec<String> = vec![];
    // HACK
    for i in 0..=100000 {
        let mut board: Vec<Vec<char>> = contents.iter().map(|s| s.chars().collect()).collect();
        for rn in 0..board.len() {
            if split_left == true {
                // dbg!("split left is true");
                for cn in 0..board[0].len() {
                    match board[rn - 1][cn] {
                        // 'S' => board[rn + 1][cn] = '|',
                        // '^' => beam_quantum(&mut board, rn, cn, &mut split_left),
                        '|' => beam_down_alt(&mut board, rn, cn),
                        _ => (),
                    }
                }

                split_left = false;
            }
            for cn in 0..board[0].len() {
                match board[rn][cn] {
                    'S' => board[rn + 1][cn] = '|',
                    '^' => beam_quantum(&mut board, rn, cn, &mut split_left),
                    '|' => beam_down(&mut board, rn, cn),
                    _ => (),
                }
            }
        }
        let pretty_board: Vec<String> = board.iter().map(|l| l.iter().collect()).collect();
        let pretty_board: String = pretty_board.join("\n");
        // println!("{}", &pretty_board);
        if !all_boards.contains(&pretty_board) {
            all_boards.push(pretty_board);
        }
    }
    // println!("{:?}", &all_boards);
    all_boards.iter().count()
}

fn main() {
    println!("Timelines: {:?}", part_two("test_input.txt"));
}
