use std::fs;

#[derive(Debug)]
struct Operation {
    nums: Vec<isize>,
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
    // String to vector of string
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

fn main() {
    // parse("test_input.txt");
    println!("Sum: {:?}", parse("input.txt"));
}
