use std::fs;
fn parse(path: &str) -> usize {
    let contents = fs::read_to_string(path).expect("Where's the file nanoda?");
    let mut contents: Vec<&str> = contents.lines().collect();
    let mut index = 0;
    for n in 0..contents.len() {
        if contents[n] == "" {
            index = n;
        }
    }
    let ids = contents.split_off(index + 1);
    let ids: Vec<usize> = ids.iter().map(|s| s.parse().unwrap()).collect();
    // let id_ranges = contents.split_off(at)
    let mut id_ranges = contents.clone();
    id_ranges.pop();
    let id_ranges: Vec<Vec<&str>> = id_ranges
        .iter()
        .map(|s| s.split("-").collect::<Vec<&str>>())
        .collect();
    let id_ranges: Vec<(usize, usize)> = id_ranges
        .clone()
        .iter()
        .map(|r| {
            let i1 = r[0].parse::<usize>().unwrap();
            let i2 = r[1].parse::<usize>().unwrap();
            (i1, i2)
        })
        .collect();
    let fresh_ids: Vec<usize> = ids
        .iter()
        .filter(|id| {
            id_ranges
                .iter()
                .any(|range| **id >= range.0 && **id <= range.1)
        })
        .copied()
        .collect();

    // dbg!(&id_ranges);
    // dbg!(&ids);
    // dbg!(&fresh_ids);
    fresh_ids.iter().count()
}

fn main() {
    let fresh_count = parse("input.txt");
    println!("Fresh: {fresh_count}");
}
