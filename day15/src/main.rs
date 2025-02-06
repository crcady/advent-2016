fn main() {
    let mut start_time = 0;
    let disks = make_disks();

    'outer: loop {
        for d in &disks {
            if !d.check_time(start_time) {
                start_time += 1;
                continue 'outer;
            }
        }
        break;
    }
    println!("Earliest good time is {}", start_time);
}

struct Disc {
    id: u32,
    positions: u32,
    start: u32,
}

impl Disc {
    fn check_time(&self, t: u32) -> bool {
        let arrival_time = self.id + t;
        let pos = (self.start + arrival_time) % self.positions;
        pos == 0
    }
}

#[test]
fn test_disc() {
    let disk1 = Disc {
        id: 1,
        positions: 5,
        start: 4,
    };

    let disk2 = Disc {
        id: 2,
        positions: 2,
        start: 1,
    };

    assert_eq!(disk1.check_time(0), true);
    assert_eq!(disk2.check_time(0), false);

    assert_eq!(disk1.check_time(5), true);
    assert_eq!(disk1.check_time(5), true);
}

fn make_disks() -> Vec<Disc> {
    let mut res = Vec::new();
    res.push( Disc {
        id: 1,
        positions: 5,
        start: 2,
    });

    res.push( Disc {
        id: 2,
        positions: 13,
        start: 7,
    });

    res.push( Disc {
        id: 3,
        positions: 17,
        start: 10,
    });

    res.push( Disc {
        id: 4,
        positions: 3,
        start: 2,
    });

    res.push( Disc {
        id: 5,
        positions: 19,
        start: 9,
    });

    res.push( Disc {
        id: 6,
        positions: 7,
        start: 0,
    });

    res.push( Disc {
        id: 7,
        positions: 11,
        start: 0,
    });

    res
}