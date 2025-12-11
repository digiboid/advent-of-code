use std::fs;

#[derive(Debug)]
struct IDRange {
    first_id: u64,
    last_id: u64,
}
fn sum_up(vec_idranges: Vec<IDRange>) -> u64 {
    vec_idranges
        .into_iter()
        .map(|idrange| get_invalid_complex(idrange).iter().copied().sum::<u64>())
        .sum::<u64>()
}

fn check_id(id: u64) -> bool {
    let string_id = id.to_string();
    let vec_id: Vec<u32> = string_id.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for n in 1..=string_id.len() / 2 {
        let chunks: Vec<Vec<u32>> = vec_id.chunks(n).map(|chunk| chunk.to_vec()).collect();
        let first_chunk = chunks[0].clone();
        if chunks.iter().cloned().all(|chunk| chunk == first_chunk) {
            return true;
        }
    }
    false
}

fn get_invalid_complex(idrange: IDRange) -> Vec<u64> {
    let mut invalid_ids: Vec<u64> = vec![];
    for id in idrange.first_id..=idrange.last_id {
        if check_id(id) {
            invalid_ids.push(id);
        }
    }
    return invalid_ids;
}

#[allow(dead_code)]
fn get_invalid(idrange: IDRange) -> Vec<u64> {
    let mut invalid_ids: Vec<u64> = vec![];
    for id in idrange.first_id..=idrange.last_id {
        let stringified_id = id.to_string();
        if stringified_id.len() % 2 == 0 {
            let split_id = stringified_id.split_at(stringified_id.len() / 2);
            let split_first: u64 = split_id.0.parse().unwrap();
            let split_second: u64 = split_id.1.parse().unwrap();
            if split_first == split_second {
                invalid_ids.push(id);
            }
        }
    }
    invalid_ids
    // return vec![11, 22];
}

fn parse(filepath: String) -> Vec<IDRange> {
    let contents = fs::read_to_string(filepath).unwrap();
    let unclean_string = contents.lines().collect::<String>();
    let unclean_ranges = unclean_string.split(',').collect::<Vec<&str>>();
    let id_ranges: Vec<IDRange> = unclean_ranges
        .iter()
        .map(|range_str| {
            let split_ranges = range_str.split_once('-').unwrap();
            IDRange {
                first_id: split_ranges.0.parse().unwrap(),
                last_id: split_ranges.1.parse().unwrap(),
            }
        })
        .collect();
    id_ranges
}
fn main() {
    let sum = sum_up(parse(String::from("input.txt")));
    println!("{:?}", sum)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn passes_test() {
        let filepath = String::from("test_input.txt");
        let id_ranges = parse(filepath);
        assert_eq!(sum_up(id_ranges), 1227775554)
    }
    #[test]
    #[ignore]
    fn does_anything() {
        let filepath = String::from("test_input.txt");
        let id_ranges = parse(filepath);
        sum_up(id_ranges);
    }
    #[test]
    fn gets_ids() {
        let idrange = IDRange {
            first_id: 95,
            last_id: 115,
        };
        assert_eq!(get_invalid_complex(idrange), vec![99, 111]);
    }
}
