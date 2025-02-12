use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut m = Machine::new();
    for line in io::stdin().lock().lines() {
        m.add_instr(parse_line(&line.unwrap()));
    }
    println!("Finished parsing program...");
    while m.step() {}

    println!(
        "The value of register a is {}",
        m.regfile.get(&'a').unwrap()
    );
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Operation,
    arguments: Vec<Operand>,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Cpy,
    Inc,
    Dec,
    Jnz,
    Tgl,
}

impl Operation {
    fn check_args(&self, args: &Vec<Operand>) -> bool {
        match self {
            Operation::Cpy => args.len() == 2 && matches!(args[1], Operand::Reg(_)),
            Operation::Inc | Operation::Dec => {
                args.len() == 1 && matches!(args[0], Operand::Reg(_))
            }
            Operation::Jnz => args.len() == 2,
            Operation::Tgl => args.len() == 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Reg(char),
    Imm(i32),
}

impl Operand {
    fn from(s: &str) -> Self {
        let num: Result<i32, _> = s.parse();

        if num.is_ok() {
            Self::Imm(num.unwrap())
        } else {
            Self::Reg(s.chars().next().unwrap())
        }
    }
}

struct Machine {
    ip: usize,
    regfile: HashMap<char, i32>,
    instructions: Vec<Instruction>,
}

impl Machine {
    fn new() -> Self {
        let mut regfile = HashMap::new();
        regfile.insert('a', 0);
        regfile.insert('b', 0);
        regfile.insert('c', 0);
        regfile.insert('d', 0);

        Self {
            ip: 0,
            regfile,
            instructions: Vec::new(),
        }
    }

    fn add_instr(&mut self, ins: Instruction) {
        self.instructions.push(ins);
    }

    fn get_value(&self, operand: &Operand) -> i32 {
        match operand {
            Operand::Reg(reg) => *self.regfile.get(reg).unwrap(),
            Operand::Imm(val) => *val,
        }
    }

    fn step(&mut self) -> bool {
        // Returns true if the machine can keep going
        if !self.instructions[self.ip]
            .op
            .check_args(&self.instructions[self.ip].arguments)
        {
            self.ip += 1;
            return self.ip < self.instructions.len();
        }

        // We can't keep a reference to anything in self.instructions because
        // we need to mutate instructions, including possibly the current instruction
        let op = self.instructions[self.ip].op.clone();

        match op {
            Operation::Cpy => {
                let Operand::Reg(dst) = self.instructions[self.ip].arguments[1] else {
                    unreachable!()
                };

                let val = self.get_value(&self.instructions[self.ip].arguments[0]);
                self.regfile.insert(dst, val);
                self.ip += 1;
            }
            Operation::Inc => {
                let Operand::Reg(dst) = self.instructions[self.ip].arguments[0] else {
                    unreachable!()
                };

                let old = self.regfile.get(&dst).unwrap();

                self.regfile.insert(dst, old + 1);
                self.ip += 1;
            }
            Operation::Dec => {
                let Operand::Reg(dst) = self.instructions[self.ip].arguments[0] else {
                    unreachable!()
                };

                let old = self.regfile.get(&dst).unwrap();

                self.regfile.insert(dst, old - 1);
                self.ip += 1;
            }
            Operation::Jnz => {
                let x = self.get_value(&self.instructions[self.ip].arguments[0]);
                if x != 0 {
                    let y = self.get_value(&self.instructions[self.ip].arguments[1]);
                    self.ip = ((self.ip as i32) + y) as usize;
                } else {
                    self.ip += 1;
                }
            }
            Operation::Tgl => {
                let offset = self.get_value(&self.instructions[self.ip].arguments[0]);
                let target = (self.ip as i32) + offset;
                if target < 0 {
                    self.ip += 1;
                } else {
                    let target = target as usize;
                    if target >= self.instructions.len() {
                        self.ip += 1;
                    } else {
                        let oldop = self.instructions[target].op.clone();
                        self.instructions[target].op = match oldop {
                            Operation::Cpy => Operation::Jnz,
                            Operation::Inc => Operation::Dec,
                            Operation::Dec => Operation::Inc,
                            Operation::Jnz => Operation::Cpy,
                            Operation::Tgl => Operation::Inc,
                        };
                        self.ip += 1;
                    }
                }
            }
        };

        self.ip < self.instructions.len()
    }
}

fn parse_line(line: &str) -> Instruction {
    let splits: Vec<_> = line.split_whitespace().collect();
    match splits[0] {
        "cpy" => Instruction {
            op: Operation::Cpy,
            arguments: vec![Operand::from(splits[1]), Operand::from(splits[2])],
        },
        "inc" => Instruction {
            op: Operation::Inc,
            arguments: vec![Operand::from(splits[1])],
        },
        "dec" => Instruction {
            op: Operation::Dec,
            arguments: vec![Operand::from(splits[1])],
        },
        "jnz" => Instruction {
            op: Operation::Jnz,
            arguments: vec![Operand::from(splits[1]), Operand::from(splits[2])],
        },
        "tgl" => Instruction {
            op: Operation::Tgl,
            arguments: vec![Operand::from(splits[1])],
        },
        _ => panic!("Unexpected opcode"),
    }
}
