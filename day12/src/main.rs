use std::{collections::HashMap, io::{self, BufRead}};

fn main() {
    let program: Vec<Instruction> = io::stdin()
    .lock()
    .lines()
    .map(|x| parse_line(&x.unwrap()))
    .collect();

    let mut m = Machine::new(program);

    while m.step() {}

    println!("The value of register a is: {}", m.registers[&'a'])
}

struct Machine {
    registers: HashMap<char, i32>,
    program: Vec<Instruction>,
    ip: usize,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        let mut registers = HashMap::new();
        registers.insert('a', 0);
        registers.insert('b', 0);
        registers.insert('c', 0);
        registers.insert('d', 0);

        Self {
            registers,
            program,
            ip: 0,
        }
    }

    fn step(&mut self) -> bool {
        // Returns true if the program can keep going
        match &self.program[self.ip] {
            Instruction::Copy(source, dest) => {
                let val: i32 = match source {
                    Source::Immediate(x) => *x,
                    Source::Register(reg) => self.registers[reg],
                };
                self.registers.insert(*dest, val);
                self.ip += 1;
            },
            Instruction::Inc(reg) => {
                let current = self.registers[reg];
                self.registers.insert(*reg, current + 1);
                self.ip += 1;
            },
            Instruction::Dec(reg) => {
                let current = self.registers[reg];
                self.registers.insert(*reg, current - 1);
                self.ip += 1;
            },
            Instruction::Jnz(source, offset) => {
                let val: i32 = match source {
                    Source::Immediate(x) => *x,
                    Source::Register(reg) => self.registers[reg],
                };

                self.ip = match val == 0 {
                    true => self.ip + 1,
                    false => (self.ip as i32 + offset) as usize,
                };
            },
            Instruction::Invalid => unimplemented!(),
        }

        self.ip < self.program.len()
    }
}
#[derive(Debug, PartialEq)]
enum Source {
    Immediate(i32),
    Register(char),
}

enum Instruction {
    Copy(Source, char),
    Inc(char),
    Dec(char),
    Jnz(Source, i32),
    Invalid,
}

fn parse_line(line: &str) -> Instruction {
    let splits: Vec<_> = line.split(" ").collect();

    match splits[0] {
        "cpy" => Instruction::Copy(
            match splits[1] {
                "a" | "b" | "c" | "d" => Source::Register(splits[1].chars().next().unwrap()),
                imm => Source::Immediate(imm.parse().unwrap()),
            },
            splits[2].chars().next().unwrap(),
        ),
        "inc" => Instruction::Inc(splits[1].chars().next().unwrap()),
        "dec" => Instruction::Dec(splits[1].chars().next().unwrap()),
        "jnz" => Instruction::Jnz(
            match splits[1] {
                "a" | "b" | "c" | "d" => Source::Register(splits[1].chars().next().unwrap()),
                imm => Source::Immediate(imm.parse().unwrap()),
            },
            splits[2].parse().unwrap(),
        ),
        _ => Instruction::Invalid,
    }
}

#[test]
fn test_parse() {
    let ins = parse_line("cpy 41 a");
    if let Instruction::Copy(src, dst) = ins {
        assert_eq!(dst, 'a');
        assert_eq!(src, Source::Immediate(41));
    } else {
        assert!(false, "Wrong type of Instruction returned");
    }
}
