fn main() {
    let mut tr = TrapRoom::new(".^..^....^....^^.^^.^.^^.^.....^.^..^...^^^^^^.^^^^.^.^^^^^^^.^^^^^..^.^^^.^^..^.^^.^....^.^...^^.^.");
    while tr.rows < 400_000 {
        tr.step();
    }

    println!("Found {} safe tiles", tr.safe_tiles);
}

struct TrapRoom {
    rows: usize,
    safe_tiles: usize,
    current_row: Vec<bool>,
}

impl TrapRoom {
    fn new(line: &str) -> Self {
        let mut current_row = Vec::new();
        for c in line.chars() {
            current_row.push(match c {
                '.' => true,
                '^' => false,
                _ => unreachable!(),
            });
        }

        let safe_tiles = current_row.iter().filter(|x|**x).collect::<Vec<_>>().len();

        Self { rows: 1, safe_tiles, current_row }
    }

    fn step(&mut self) {
        let mut new_row = Vec::new();

        for i in 0..self.current_row.len() {
            let left = self.current_row.get(i-1).unwrap_or(&true);
            let center = &self.current_row[i];
            let right = self.current_row.get(i+1).unwrap_or(&true);

            new_row.push(match [left, center, right] {
                [false, false, true] => false,
                [true, false, false] => false,
                [false, true, true] => false,
                [true, true, false] => false,
                _ => true
            });
        }

        self.safe_tiles += new_row.iter().filter(|x|**x).collect::<Vec<_>>().len();
        self.current_row = new_row;
        self.rows += 1;
    }
}

#[test]
fn test_trap_room() {
    let mut tr = TrapRoom::new("..^^.");
    assert_eq!(tr.safe_tiles, 3);

    tr.step();
    assert_eq!(tr.current_row.len(), 5);
    assert_eq!(tr.current_row, vec![true, false, false, false, false]);
    assert_eq!(tr.safe_tiles, 4);
}