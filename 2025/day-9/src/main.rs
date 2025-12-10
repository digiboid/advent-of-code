use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};
use std::fs;

fn part_one(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let (_, red_tiles) = parse(&contents).unwrap();
    let max = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            (a.x.abs_diff(b.x) + 1)
                * (a.y.abs_diff(b.y) + 1)
        })
        .max()
        .unwrap();
    println!("{:?}", max)
}

fn parse(path: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::i64,
            tag(","),
            complete::i64,
        )
        .map(|(x, y)| I64Vec2::new(x, y)),
    )
    .parse(path)
}
fn main() {
    part_one("input.txt");
}
