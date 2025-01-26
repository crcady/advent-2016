use std::io::{self, BufRead};

fn main() {
    // let mut sc = Screen::new();
    // let instr = Instruction::from_line("rect 3x2");
    // sc.process(&instr);
    // sc.display();

    // println!("---");

    // let instr = Instruction::from_line("rotate row y=1 by 4");
    // sc.process(&instr);
    // sc.display();

    // println!("---");

    // let instr = Instruction::from_line("rotate column x=0 by 1");
    // sc.process(&instr);
    // sc.display();

    // println!("---");

    // let instr = Instruction::from_line("rotate row y=0 by 48");
    // sc.process(&instr);
    // sc.display();

    // println!("---");
    // let instr = Instruction::from_line("rotate column x=0 by 5");
    // sc.process(&instr);
    // sc.display();

    let instrs: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|x| Instruction::from_line(&x.unwrap()))
        .collect();
    let mut sc = Screen::new();
    for instr in &instrs {
        sc.process(instr);
    }

    println!("There are {} pixels activated", sc.count());
    sc.display();
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Rect,
    RCol,
    RRow,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    arg1: usize,
    arg2: usize,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let splits: Vec<_> = line.split(" ").collect();
        match splits.len() {
            2 => {
                let args: Vec<_> = splits[1].split("x").collect();
                Instruction {
                    opcode: Opcode::Rect,
                    arg1: args[0]
                        .parse()
                        .expect("Couldn't parse rectangle's first arg to usize"),
                    arg2: args[1]
                        .parse()
                        .expect("Couldn't parse rectangle's second arg to usize"),
                }
            }
            5 => {
                let opcode = match splits[1] {
                    "row" => Opcode::RRow,
                    "column" => Opcode::RCol,
                    _ => panic!("Couldn't parse the rotation target"),
                };

                let target: Vec<_> = splits[2].split("=").collect();

                Instruction {
                    opcode,
                    arg1: target[1].parse().expect("Couldn't parse rotation target"),
                    arg2: splits[4].parse().expect("Couldn't parse rotation amount"),
                }
            }
            _ => panic!("Couldn't parse line into Instruction"),
        }
    }
}

struct Screen {
    rows: Vec<Vec<bool>>,
}

impl Screen {
    fn new() -> Self {
        Self {
            rows: vec![vec![false; 50]; 6],
        }
    }

    fn display(&self) {
        for row in &self.rows {
            for pixel in row {
                print!(
                    "{}",
                    match pixel {
                        true => "#",
                        false => ".",
                    }
                );
            }
            print!("\n");
        }
    }

    fn count(&self) -> usize {
        self.rows.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|x| **x).collect::<Vec<_>>().len()
        })
    }

    fn process(&mut self, instr: &Instruction) {
        match instr.opcode {
            Opcode::Rect => {
                for col in 0..instr.arg1 {
                    for row in 0..instr.arg2 {
                        self.rows[row][col] = true;
                    }
                }
            }
            Opcode::RCol => {
                let target_col = instr.arg1;
                let shift_amt = instr.arg2 % 6;

                // Save the bottom-most `shift_amt` pixels into a temporary buffer
                let mut temp: Vec<bool> = Vec::new();
                for r in 6 - shift_amt..6 {
                    temp.push(self.rows[r][target_col])
                }

                // Rotate the pixels down
                for i in 0..6 - shift_amt {
                    // Math is hard, write it out and let the compiler sort it out
                    let dst_row = 5 - i;
                    let src_row = dst_row - shift_amt;

                    self.rows[dst_row][target_col] = self.rows[src_row][target_col];
                }

                // Copy the buffer back to the top
                for r in 0..shift_amt {
                    self.rows[r][target_col] = temp[r];
                }
            }
            Opcode::RRow => {
                let target_row = instr.arg1;
                let shift_amt = instr.arg2 % 50;

                // Save the right-most `shift_amt` pixels into a temporary buffer
                let mut temp: Vec<bool> = Vec::new();
                for p in &self.rows[target_row][50 - shift_amt..50] {
                    temp.push(*p);
                }

                // Rotate the pixels over
                for i in 0..50 - shift_amt {
                    // Math is hard, so we'll just let the compiler optimize out our intermediate values
                    let dst = 49 - i;
                    let src = dst - shift_amt;

                    self.rows[target_row][dst] = self.rows[target_row][src].clone();
                    // Clone makes the compiler happy since we're in the same vector
                }

                // Copy the buffer back over
                for i in 0..shift_amt {
                    self.rows[target_row][i] = temp[i];
                }
            }
        }
    }
}

#[test]
fn test_make_instruction() {
    let rect = Instruction::from_line("rect 2x3");
    assert_eq!(rect.opcode, Opcode::Rect);
    assert_eq!(rect.arg1, 2);
    assert_eq!(rect.arg2, 3);

    let row = Instruction::from_line("rotate row y=2 by 10");
    assert_eq!(row.opcode, Opcode::RRow);
    assert_eq!(row.arg1, 2);
    assert_eq!(row.arg2, 10);

    let col = Instruction::from_line("rotate column x=0 by 1");
    assert_eq!(col.opcode, Opcode::RCol);
    assert_eq!(col.arg1, 0);

    assert_eq!(col.arg2, 1);
}

#[test]
fn test_count() {
    let mut sc = Screen::new();
    assert_eq!(sc.count(), 0);

    let rect = Instruction::from_line("rect 2x3");
    sc.process(&rect);
    assert_eq!(sc.count(), 6);
}