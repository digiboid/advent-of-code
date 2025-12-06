use std::fs;

#[derive(Debug)]
struct Operation {
    nums: Vec<isize>,
    opr: char,
}

#[derive(Debug)]
struct PreOperation {
    digits: Vec<String>,
    opr: char,
}

fn calc_operation(opr: &Operation) -> isize {
    match opr.opr {
        '+' => opr.nums.iter().sum(),
        '*' => opr.nums.iter().product(),
        _ => panic!("知らんけど"),
    }
}

fn part_one(path: &str) -> isize {
    let contents = fs::read_to_string(path).expect("No file");
    let contents: Vec<&str> = contents.lines().collect();
    let contents: Vec<Vec<&str>> = contents
        .iter()
        .map(|x| x.split_whitespace().collect())
        .collect();
    let mut columns: Vec<Vec<&str>> = vec![];
    for n in 0..contents[0].len() {
        let mut column: Vec<&str> = vec![];
        for i in 0..contents.len() {
            column.push(contents[i][n]);
        }
        columns.push(column);
    }
    let columns: Vec<Operation> = columns
        .iter()
        .map(|col| {
            let mut nums = col.clone();
            nums.truncate(col.len() - 1);
            let nums = nums.iter().map(|n| n.parse().unwrap()).collect();
            Operation {
                opr: col.last().unwrap().parse::<char>().unwrap(),
                nums,
            }
        })
        .collect();
    let results: Vec<isize> = columns.iter().map(|c| calc_operation(c)).collect();
    results.iter().sum()
}

fn gather_operation(cidx: usize, opr_len: usize, contents: Vec<Vec<char>>) -> Operation {
    let opr = contents.last().unwrap()[cidx - opr_len];
    let mut nums = contents.clone();
    let mut all_nums: Vec<isize> = vec![];
    nums.truncate(contents.len() - 1);

    for n in (1..=opr_len) {
        let mut single_num: Vec<char> = vec![];
        for line in nums.clone() {
            single_num.push(line[cidx - n]);
        }
        let mut single_num: String = single_num.iter().collect();
        let single_num = single_num.trim();
        let single_num: usize = single_num.parse().unwrap();
        all_nums.push(single_num as isize);
    }

    Operation {
        nums: all_nums,
        opr,
    }
}

fn part_two(path: &str) -> isize {
    let contents = fs::read_to_string(path).expect("No file");
    let contents: Vec<&str> = contents.lines().collect();
    let contents: Vec<String> = contents.iter().map(|l| l.to_string() + " ").collect();
    let contents: Vec<Vec<char>> = contents.iter().map(|l| l.chars().collect()).collect();
    let mut operations: Vec<Operation> = vec![];
    let mut opr_len = 0;
    for cidx in 0..contents[0].len() {
        if !contents.iter().all(|l| l[cidx] == ' ') {
            opr_len += 1;
        } else {
            operations.push(gather_operation(cidx, opr_len, contents.clone()));
            opr_len = 0;
        }
    }

    let results: Vec<isize> = operations.iter().map(|c| calc_operation(c)).collect();
    results.iter().sum()
}

fn main() {
    println!("Sum: {:?}", part_two("input.txt"));
}
