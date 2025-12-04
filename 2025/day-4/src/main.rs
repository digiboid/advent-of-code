use std::{clone, fs};

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
    let max_row = warehouse.row.len();
    for row in 0..max_row {
        let max_col = warehouse.row[row].column.len();
        for column in 0..max_col {
            warehouse.row[row].column[column].adjacent == 0;
            if row > 0 && column > 0 {
                if warehouse.row[row - 1].column[column - 1].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
                }
            }
            if column > 0 {
                if warehouse.row[row].column[column - 1].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
                }
            }
            if row + 1 < max_row && column > 0 {
                if warehouse.row[row + 1].column[column - 1].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
                }
            }
            if row > 0 {
                if warehouse.row[row - 1].column[column].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
                }
            }
            if row + 1 < max_row {
                if warehouse.row[row + 1].column[column].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
                }
            }
            if row > 0 && column + 1 < max_col {
                if warehouse.row[row - 1].column[column + 1].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
                }
            }
            if column + 1 < max_col {
                if warehouse.row[row].column[column + 1].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
                }
            }
            if row + 1 < max_row && column + 1 < max_col {
                if warehouse.row[row + 1].column[column + 1].item == Some(Paper)
                    && warehouse.row[row].column[column].item == Some(Paper)
                {
                    warehouse.row[row].column[column].adjacent += 1
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

fn main() {
    let warehouse = parse(String::from("input.txt"));
    let warehouse = fill_adjacent(warehouse);
    let removable = count_removable(&warehouse);
    println!("{}", removable);
}
