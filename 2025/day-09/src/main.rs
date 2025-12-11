use glam::I64Vec2;
use itertools::{Itertools, rev};
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
    println!("{:?}", max);
}

fn part_two(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let (_, red_tiles) = parse(&contents).unwrap();
    let lines = red_tiles
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&I64Vec2, &I64Vec2)>>();
    let max = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let area = (a.x.abs_diff(b.x) + 1)
                * (a.y.abs_diff(b.y) + 1);
            (a, b, area)
        })
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(a, b, area)| {
            lines.iter().all(|(line_start, line_end)| {
                let left_of_rect = a.x.max(b.x)
                    <= line_start.x.min(line_end.x);
                let right_of_rect = a.x.min(b.x)
                    >= line_start.x.max(line_end.x);
                let above = a.y.max(b.y)
                    <= line_start.y.min(line_end.y);
                let below = a.y.min(b.y)
                    >= line_start.y.max(line_end.y);
                left_of_rect
                    || right_of_rect
                    || above
                    || below
            })
        });

    println!("{:?}", max.unwrap().2);
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
    part_two("input.txt");
}
