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

    let mut id_ranges = id_ranges;
    id_ranges.dedup();
    id_ranges.sort_by_key(|r| r.0);
    let mut merged_ranges: Vec<(usize, usize)> = vec![];
    let rem_ranges = id_ranges.split_off(1);
    merged_ranges.push(id_ranges[0]);
    for range in rem_ranges.clone() {
        let last = merged_ranges.last_mut().unwrap();
        if range.0 <= last.1 + 1 {
            if last.1 < range.1 {
                last.1 = range.1
            }
        } else {
            merged_ranges.push(range);
        }
    }

    // for range in rem_ranges.clone() {
    //     if let Some(last) = merged_ranges.last_mut() {
    //         if range.0 <= last.1 + 1 {
    //             last.1 = last.1.max(range.1);
    //         } else {
    //             merged_ranges.push(range);
    //         }
    //     } else {
    //         merged_ranges.push(range);
    //     }
    // }

    let all_fresh_count: Vec<usize> = merged_ranges
        .iter()
        .map(|range| range.1 - range.0 + 1)
        .collect();
    all_fresh_count.iter().sum()
}

fn main() {
    let fresh_count = parse("input.txt");
    println!("Fresh: {fresh_count}");
}
