use glam::IVec3;
use itertools::Itertools;
use std::{fs, path::Component};
use tracing::info;

fn part_one(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let contents: Vec<&str> = contents.lines().collect();
    let contents: Vec<Vec<&str>> = contents
        .iter()
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .collect();
    let contents: Vec<Vec<i32>> = contents
        .iter()
        .map(|v| v.iter().map(|l| l.parse().unwrap()).collect())
        .collect();
    let positions: Vec<IVec3> = contents
        .iter()
        .map(|v| IVec3 {
            x: v[0],
            y: v[1],
            z: v[2],
        })
        .collect();
    let mut connections: Vec<Vec<IVec3>> = vec![];
    for (a, b, _) in positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .take(1000)
    {
        let matches = connections
            .iter()
            .positions(|cluster| {
                let contains_a = cluster.contains(a);
                let contains_b = cluster.contains(b);
                contains_a || contains_b
            })
            .collect::<Vec<usize>>();
        match matches.as_slice() {
            [] => {
                connections.push(vec![*a, *b]);
            }
            [index] => {
                let cluster = connections.get_mut(*index).unwrap();
                let contains_a = cluster.contains(a);
                let contains_b = cluster.contains(b);
                match (contains_a, contains_b) {
                    (true, true) => {}
                    (true, false) => {
                        cluster.push(*b);
                    }
                    (false, true) => {
                        cluster.push(*a);
                    }
                    (false, false) => {
                        panic!("Shouldn't happen");
                    }
                }
            }
            [index_a, index_b] => {
                let a = connections.remove(*index_a.max(index_b));
                let b = connections.remove(*index_a.min(index_b));
                let new_cluster = a
                    .into_iter()
                    .chain(b.into_iter())
                    .unique()
                    .collect::<Vec<IVec3>>();
                connections.push(new_cluster);
            }
            _ => {
                panic!("");
            }
        }
    }

    connections.sort_by(|a, b| b.len().cmp(&a.len()));
    let t: usize = connections.iter().map(|v| v.len()).take(3).product();
    dbg!(t);
}

fn part_two(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let contents: Vec<&str> = contents.lines().collect();
    let contents: Vec<Vec<&str>> = contents
        .iter()
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .collect();
    let contents: Vec<Vec<i32>> = contents
        .iter()
        .map(|v| v.iter().map(|l| l.parse().unwrap()).collect())
        .collect();
    let positions: Vec<IVec3> = contents
        .iter()
        .map(|v| IVec3 {
            x: v[0],
            y: v[1],
            z: v[2],
        })
        .collect();
    let mut connections: Vec<Vec<IVec3>> = vec![];
    for (a, b, _) in positions
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    {
        let matches = connections
            .iter()
            .positions(|cluster| {
                let contains_a = cluster.contains(a);
                let contains_b = cluster.contains(b);
                contains_a || contains_b
            })
            .collect::<Vec<usize>>();
        match matches.as_slice() {
            [] => {
                connections.push(vec![*a, *b]);
                if connections.len() == 1 && connections[0].len() == positions.len() {
                    // new cluster includes all positions
                    dbg!(a.x as usize * b.x as usize);
                }
            }
            [index] => {
                let cluster = connections.get_mut(*index).unwrap();
                let contains_a = cluster.contains(a);
                let contains_b = cluster.contains(b);
                // cluster contains one of the junction boxes
                match (contains_a, contains_b) {
                    (true, true) => {
                        // do nothing, both are already in the cluster
                    }
                    (true, false) => {
                        cluster.push(*b);
                        if connections.len() == 1 && connections[0].len() == positions.len() {
                            // new cluster includes all positions
                            dbg!(a.x as usize * b.x as usize);
                        }
                    }
                    (false, true) => {
                        cluster.push(*a);
                        if connections.len() == 1 && connections[0].len() == positions.len() {
                            // new cluster includes all positions
                            dbg!(a.x as usize * b.x as usize);
                        }
                    }
                    (false, false) => {
                        panic!("We just filtered for a truth, so this should never happen");
                    }
                }
            }
            [index_a, index_b] => {
                let a_group = connections.remove(*index_a.max(index_b));
                let b_group = connections.remove(*index_a.min(index_b));
                let new_cluster = a_group
                    .into_iter()
                    .chain(b_group.into_iter())
                    .unique()
                    .collect::<Vec<IVec3>>();
                if new_cluster.len() == positions.len() {
                    // new cluster includes all positions
                    dbg!(a.x as usize * b.x as usize);
                }
                connections.push(new_cluster);
            }
            _ => {
                panic!("");
            }
        }
    }

    info!(
        conn_len = connections[0].len(),
        connections=?connections[0],
        pos_len = positions.len(),
        ?positions
    );
}
fn main() {
    println!("{:?}", part_two("input.txt"));
}
