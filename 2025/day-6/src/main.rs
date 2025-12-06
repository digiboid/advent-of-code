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

// fn add_nums(nums: Vec<isize>) -> isize {
//     nums.iter().reduce(|&acc, &e| acc + e).unwrap()
// }

fn calc_operation(opr: &Operation) -> isize {
    match opr.opr {
        '+' => opr.nums.iter().sum(),
        '*' => opr.nums.iter().product(),
        _ => panic!("知らんけど"),
    }
}

fn parse(path: &str) -> isize {
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
        // dbg!(&columns);
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
    // dbg!(&columns);
    let results: Vec<isize> = columns.iter().map(|c| calc_operation(c)).collect();
    // dbg!(&results);
    results.iter().sum()
    // dbg!(&contents);
}

fn gather_operation(cidx: usize, opr_len: usize, contents: Vec<Vec<char>>) -> Operation {
    let opr = contents.last().unwrap()[cidx - opr_len];
    // dbg!(&opr);
    let mut nums = contents.clone();
    let mut all_nums: Vec<isize> = vec![];
    nums.truncate(contents.len() - 1);

    for n in (1..=opr_len) {
        let mut single_num: Vec<char> = vec![];
        for line in nums.clone() {
            single_num.push(line[cidx - n]);
            // dbg!(&single_num);
        }
        let mut single_num: String = single_num.iter().collect();
        let single_num = single_num.trim();
        let single_num: usize = single_num.parse().unwrap();
        all_nums.push(single_num as isize);
    }

    // for dr in (1..=opr_len) {
    //     for dc in (1..=opr_len).rev() {
    //         dbg!(&nums[contents.len() - 1 - dc]);
    //         single_num.push(nums[contents.len() - dc][cidx - dr]);
    //     }
    //     // dbg!(&single_num);
    //     all_nums.push(single_num as isize);
    // }
    // dbg!(&all_nums);
    Operation {
        nums: all_nums,
        opr,
    }
}

fn part_two(path: &str) -> isize {
    let contents = fs::read_to_string(path).expect("No file");
    let contents: Vec<&str> = contents.lines().collect();
    let contents: Vec<String> = contents.iter().map(|l| l.to_string() + " ").collect();
    // dbg!(&contents);
    let contents: Vec<Vec<char>> = contents.iter().map(|l| l.chars().collect()).collect();
    // dbg!(&contents[0]);
    // dbg!(&contents[0].len());
    let mut operations: Vec<Operation> = vec![];
    let mut opr_len = 0;
    for cidx in 0..contents[0].len() {
        // for cidx in 4..8 {
        if !contents.iter().all(|l| l[cidx] == ' ') {
            opr_len += 1;
            // dbg!(&opr_len);
        } else {
            // dbg!(&opr_len);
            operations.push(gather_operation(cidx, opr_len, contents.clone()));
            opr_len = 0;
        }
    }

    // dbg!(&operations);
    let results: Vec<isize> = operations.iter().map(|c| calc_operation(c)).collect();
    results.iter().sum()
}

fn main() {
    parse("test_input.txt");
    println!("Sum: {:?}", part_two("test_input.txt"));
}
