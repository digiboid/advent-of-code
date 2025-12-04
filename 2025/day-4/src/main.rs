use std::fs;

#[derive(Debug)]
struct Warehouse {
    row: Vec<Shelf>,
}

#[derive(Debug)]
struct Shelf {
    column: Vec<Spot>,
}

#[derive(Debug)]
struct Spot {
    item: Option<Paper>,
    adjacent: usize,
}

#[derive(Debug, PartialEq)]
struct Paper;

fn fill_adjacent(mut warehouse: Warehouse) -> Warehouse {
    let directions: Vec<(i32, i32)> = (-1..=1)
        .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        .filter(|&(dr, dc)| !(dr == 0 && dc == 0))
        .collect();
    let max_row = warehouse.row.len();
    for row in 0..max_row {
        let max_col = warehouse.row[row].column.len();
        for col in 0..max_col {
            warehouse.row[row].column[col].adjacent = 0;
            for direction in directions.clone() {
                let nrow = row as i32 + direction.0;
                let ncol = col as i32 + direction.1;
                if nrow >= 0 && nrow < max_row as i32 && ncol >= 0 && ncol < max_col as i32 {
                    let nrow = nrow as usize;
                    let ncol = ncol as usize;
                    if warehouse.row[nrow].column[ncol].item == Some(Paper)
                        && warehouse.row[row].column[col].item == Some(Paper)
                    {
                        warehouse.row[row].column[col].adjacent += 1
                    }
                }
            }
        }
    }

    return warehouse;
}

fn shelf_it(line: &str) -> Shelf {
    let line: Vec<char> = line.chars().collect();
    let line: Vec<Spot> = line
        .iter()
        .map(|c| match c {
            '.' => Spot {
                item: None,
                adjacent: 0,
            },
            '@' => Spot {
                item: Some(Paper),
                adjacent: 0,
            },
            _ => panic!(":eee:"),
        })
        .collect();
    Shelf { column: line }
}

fn parse(path: String) -> Warehouse {
    let mut warehouse = Warehouse { row: vec![] };
    let contents = fs::read_to_string(path).unwrap();
    for line in contents.lines() {
        warehouse.row.push(shelf_it(line));
    }
    return warehouse;
}

fn count_removable(warehouse: &Warehouse) -> usize {
    let mut all_removables = 0;
    for row in &warehouse.row {
        for column in &row.column {
            if column.adjacent < 4 && column.item == Some(Paper) {
                all_removables += 1
            }
        }
    }
    return all_removables;
}

fn remove_removables(mut warehouse: Warehouse) -> Warehouse {
    for row in 0..warehouse.row.len() {
        for column in 0..warehouse.row[row].column.len() {
            if warehouse.row[row].column[column].adjacent < 4
                && warehouse.row[row].column[column].item == Some(Paper)
            {
                warehouse.row[row].column[column].item = None;
            }
        }
    }
    return warehouse;
}
fn main() {
    let mut warehouse = parse(String::from("input.txt"));
    let mut removable = 0;
    loop {
        warehouse = fill_adjacent(warehouse);
        if count_removable(&warehouse) == 0 {
            break;
        }
        removable += count_removable(&warehouse);
        warehouse = remove_removables(warehouse);
    }
    println!("{}", removable);
}
