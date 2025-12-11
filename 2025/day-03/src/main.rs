// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::cmp::Ordering;
use std::fs;
use std::ops::Index;

fn find_largest(bank_vec: Vec<u64>) -> u64 {
    let mut vec = bank_vec.clone();
    vec.truncate(vec.len() - 1); // largest can't be last digit
    // The problem here is that below takes the last value
    // let max = vec.iter().enumerate().max_by_key(|&(_idx, &val)| val);
    // return max.map(|(idx, _val)| idx as u64).unwrap();
    let max = vec.iter().max().unwrap().clone();
    vec.iter().position(|&id| id == max).unwrap() as u64
}

fn find_second(bank_vec: Vec<u64>, largest: u64) -> u64 {
    let mut vec_full = bank_vec.clone();
    let vec = vec_full.split_off(largest as usize + 1);
    let max = vec.iter().enumerate().max_by_key(|&(_idx, &val)| val);
    return max.map(|(idx, _val)| idx as u64 + largest + 1).unwrap();
}

fn process_full_bank(bank: String) -> u64 {
    let mut new_vec: Vec<u64> = vec![];
    let bank_vec: Vec<u64> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    // let mut max = 0;
    let mut pos = 0;
    for n in (0..=11).rev() {
        let mut foo_vec = bank_vec.clone();
        dbg!(pos);
        let mut bar_vec = foo_vec.split_off(pos);
        bar_vec.truncate(bank_vec.len() - n - pos);

        dbg!(bar_vec.len());
        dbg!(&bar_vec);
        let max = bar_vec.iter().max().unwrap().clone();
        new_vec.push(max);
        pos += bar_vec.iter().position(|x| *x == max).unwrap() + 1;
    }
    let new_digit: u64 = new_vec
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    return new_digit;
}

fn process_bank(bank: String) -> u64 {
    let bank_vec: Vec<u64> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let largest_index = find_largest(bank_vec.clone());
    let second_index = find_second(bank_vec.clone(), largest_index);
    let largest = bank_vec[largest_index as usize].to_string();
    let second = bank_vec[second_index as usize].to_string();
    (largest + &second).parse().unwrap()
}

fn sum_up(bank_shelf: Vec<String>) -> u64 {
    let vec = bank_shelf.clone();
    let mapped_vec: Vec<u64> = vec.iter().map(|x| process_full_bank(x.clone())).collect();
    let sum_vec: u64 = mapped_vec.iter().cloned().sum();
    return sum_vec;
}

fn parse(filepath: String) -> Vec<String> {
    let contents = fs::read_to_string(filepath).unwrap();
    contents.lines().map(|x| String::from(x)).collect()
}

fn main() {
    let sum_bank = sum_up(parse(String::from("input.txt")));
    println!("Sum: {:?}", sum_bank);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn finds_largest() {
        let bank_vec = vec![8, 7, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 9];
        let largest = find_largest(bank_vec.clone());
        println!("Largest: {:?}", largest);
        println!("2nd: {:?}", find_second(bank_vec, largest));
    }
    #[test]
    #[ignore]
    fn test_bank() {
        let bank = String::from("987654321111111");
        println!("bank is: {:?}", process_bank(bank))
    }
    #[test]
    #[ignore]
    fn passes_test() {
        let sum_bank = sum_up(parse(String::from("test_input.txt")));
        assert_eq!(sum_bank, 3121910778619)
    }
    #[test]
    fn passes_full_bank() {
        let full_bank = process_full_bank(String::from("234234234234278"));
        assert_eq!(full_bank, 434234234278)
    }
}
