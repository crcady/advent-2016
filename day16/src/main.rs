fn main() {
    let mut d = Disk::new("10000", 20);
    d.make_full();
    println!("The checksum is: {}", d.checksum());
}

struct Disk {
    desired_size: usize,
    current_a: Vec<bool>,
}

impl Disk {
    fn new(a: &str, desired_size: usize) -> Self {
        let mut current_a = Vec::new();
        for c in a.chars() {
            current_a.push(match c {
                '0' => false,
                '1' => true,
                _ => unreachable!(),
            });
        }

        Self {
            desired_size,
            current_a,
        }
    }

    fn expand(&mut self) {
        let mut b = self.current_a.clone();
        b.reverse();
        self.current_a.push(false);
        self.current_a.extend(b.into_iter().map(|x| !x));
    }

    fn checksum(&self) -> String {
        let mut res = Vec::new();
        for chunk in self.current_a.chunks(2) {
            res.push(chunk[0] == chunk[1]);
        }

        while res.len() % 2 == 0 {
            let mut new_res = Vec::new();
            for chunk in res.chunks(2) {
                new_res.push(chunk[0] == chunk[1]);
            }
            res = new_res;
        }

        let mut s = String::new();
        for val in res {
            s.push(match val {
                true => '1',
                false => '0',
            });
        }

        s
    }

    fn make_full(&mut self) {
        while self.current_a.len() < self.desired_size {
            self.expand();
        }

        self.current_a = self.current_a[..self.desired_size]
            .into_iter()
            .map(|x| *x)
            .collect();
    }
}

#[test]
fn test_expand() {
    let mut d = Disk {
        desired_size: 10,
        current_a: vec![true],
    };
    d.expand();
    assert_eq!(d.current_a, vec![true, false, false]);

    let mut d = Disk {
        desired_size: 10,
        current_a: vec![false],
    };
    d.expand();
    assert_eq!(d.current_a, vec![false, false, true]);

    let mut d = Disk {
        desired_size: 10,
        current_a: vec![true; 5],
    };
    d.expand();
    let mut expected = vec![true; 5];
    let mut tail = vec![false; 6];
    expected.append(&mut tail);
    assert_eq!(d.current_a, expected);
}

#[test]
fn test_checksum() {
    let d = Disk::new("110010110100", 12);
    assert_eq!(d.checksum(), "100");
}

#[test]
fn test_make_full() {
    let mut d = Disk::new("10000", 20);
    d.make_full();
    assert_eq!(d.current_a.len(), 20);
    assert_eq!(d.checksum(), "01100");
}
