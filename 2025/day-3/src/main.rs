// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::cmp::Ordering;
use std::fs;

fn find_largest(bank_vec: Vec<u32>) -> u32 {
    let mut vec = bank_vec.clone();
    vec.truncate(vec.len() - 1); // largest can't be last digit
    let max = vec.iter().enumerate().max_by_key(|&(_idx, &val)| val);
    return max.map(|(idx, _val)| idx as u32).unwrap();
}

fn find_second(bank_vec: Vec<u32>, largest: u32) -> u32 {
    let mut vec_full = bank_vec.clone();
    let vec = vec_full.split_off(largest as usize + 1);
    let max = vec.iter().enumerate().max_by_key(|&(_idx, &val)| val);
    return max.map(|(idx, _val)| idx as u32 + largest + 1).unwrap();
}

fn process_bank(bank: String) -> u32 {
    let bank_vec: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let largest_index = find_largest(bank_vec.clone());
    let second_index = find_second(bank_vec.clone(), largest_index);
    let largest = bank_vec[largest_index as usize].to_string();
    let second = bank_vec[second_index as usize].to_string();
    (largest + &second).parse().unwrap()
}

fn sum_up(bank_shelf: Vec<String>) -> u32 {
    return 357;
}

fn parse(filepath: String) -> Vec<String> {
    let contents = fs::read_to_string(filepath).unwrap();
    contents.lines().map(|x| String::from(x)).collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_largest() {
        let bank_vec = vec![8, 7, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 9];
        let largest = find_largest(bank_vec.clone());
        println!("Largest: {:?}", largest);
        println!("2nd: {:?}", find_second(bank_vec, largest));
    }
    #[test]
    fn test_bank() {
        let bank = String::from("987654321111111");
        println!("bank is: {:?}", process_bank(bank))
    }
}
