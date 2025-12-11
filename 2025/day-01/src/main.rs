use std::fs;

struct Dial {
    digit: u32,
    zeroes: u32,
    part: u32,
}

struct Rotation {
    turn_right: bool,
    range: u32,
}

impl Dial {
    fn turn(&mut self, rotation: Rotation) {
        match rotation.turn_right {
            true => self.increment(rotation.range),
            false => self.decrement(rotation.range),
        }
    }
    fn increment(&mut self, steps: u32) {
        for _ in 0..steps {
            if self.digit == 99 {
                self.digit = 0
            } else {
                self.digit += 1;
            }
            if self.part == 2 {
                if self.digit == 0 {
                    self.zeroes += 1;
                }
            }
        }
    }
    fn decrement(&mut self, steps: u32) {
        for _ in 0..steps {
            if self.digit == 0 {
                self.digit = 99
            } else {
                self.digit -= 1;
            }
            if self.part == 2 {
                if self.digit == 0 {
                    self.zeroes += 1;
                }
            }
        }
    }
    fn check(&mut self) {
        if self.digit == 0 {
            self.zeroes += 1
        }
    }
}
fn parse(file: &String) -> Vec<Rotation> {
    let contents = fs::read_to_string(file).unwrap();
    let mut all_rotations: Vec<Rotation> = vec![];
    for line in contents.lines() {
        let turn_right = match line.chars().nth(0).unwrap() {
            'R' => true,
            'L' => false,
            _ => panic!(":eee:"),
        };
        let range: u32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        let single_rotation = Rotation { turn_right, range };
        all_rotations.push(single_rotation)
    }
    all_rotations
}

fn main() {
    let path = String::from("input.txt");
    let mut dial = Dial {
        digit: 50,
        zeroes: 0,
        part: 1,
    };
    let rotations = parse(&path);
    for rotation in rotations {
        dial.turn(rotation);
        dial.check();
    }
    println!("Part 1 zeroes: {:?}", dial.zeroes);

    let mut dial = Dial {
        digit: 50,
        zeroes: 0,
        part: 2,
    };
    let rotations = parse(&path);
    for rotation in rotations {
        dial.turn(rotation);
    }
    println!("Part 2 zeroes: {:?}", dial.zeroes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_test_one() {
        let path = String::from("test_input.txt");
        let mut dial = Dial {
            digit: 50,
            zeroes: 0,
            part: 1,
        };
        let rotations = parse(&path);
        for rotation in rotations {
            dial.turn(rotation);
            dial.check();
        }
        assert_eq!(dial.zeroes, 3)
    }

    #[test]
    fn passes_test_two() {
        let path = String::from("test_input.txt");
        let mut dial = Dial {
            digit: 50,
            zeroes: 0,
            part: 2,
        };
        let rotations = parse(&path);
        for rotation in rotations {
            dial.turn(rotation);
        }
        assert_eq!(dial.zeroes, 6)
    }
    #[test]
    fn passes_r1000() {
        let path = String::from("single_rotation.txt");
        let mut dial = Dial {
            digit: 50,
            zeroes: 0,
            part: 1,
        };
        let rotations = parse(&path);
        for rotation in rotations {
            dial.turn(rotation);
        }
        assert_eq!(dial.zeroes, 10)
    }
}
