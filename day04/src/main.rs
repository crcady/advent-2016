use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let real_rooms: Vec<_> = lines
        .iter()
        .map(|x| EncryptedRoom::from(x))
        .filter(|x| x.is_real())
        .collect();

    let ans1 = real_rooms
        .iter()
        .map(|x| x.sector_id)
        .fold(0, |acc, x| acc + x);

    println!("Sum of the real items' sector IDs is {}", ans1);

    for rr in real_rooms {
        println!("{}: {}", rr.decrypt(), rr.sector_id);
    }
}

fn shift(c: &char, amt: u32) -> char {
    let amt = amt % 26;
    char::from_u32(((((*c as u32) - ('a' as u32)) + amt) % 26) + ('a' as u32))
        .expect("Shift failed")
}

#[derive(Debug, PartialEq)]
struct EncryptedRoom {
    name: Vec<String>,
    sector_id: u32,
    checksum: String,
}

impl EncryptedRoom {
    fn from(line: &str) -> Self {
        let splits: Vec<_> = line.split("-").collect();
        let name: Vec<_> = splits[..splits.len() - 1]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let sector_id: u32 = splits[splits.len() - 1][..3]
            .parse()
            .expect("Couldn't parse sector_id");
        let checksum = splits[splits.len() - 1][4..9].to_string();

        Self {
            name,
            sector_id,
            checksum,
        }
    }

    fn is_real(&self) -> bool {
        let mut freqs: HashMap<char, i32> = HashMap::new();
        let mut letters: Vec<char> = Vec::with_capacity(26);
        for i in 0u32..26 {
            letters.push(shift(&'a', i));
        }

        for c in self.name.concat().chars() {
            freqs.insert(c, freqs.get(&c).unwrap_or(&0) + 1);
        }

        letters.sort_by_key(|k| -freqs.get(k).unwrap_or(&0));

        let checksum_chars: Vec<_> = self.checksum.chars().collect();

        for i in 0..5 {
            if letters[i] != checksum_chars[i] {
                return false;
            }
        }

        true
    }

    fn decrypt(&self) -> String {
        let shifted_name: Vec<String> = self
            .name
            .iter()
            .map(|x| String::from_iter(x.chars().map(|c| shift(&c, self.sector_id))))
            .collect();

        shifted_name.join(" ")
    }
}
#[test]
fn test_encrypted_room_from() {
    let line = "aaaaa-bbb-z-y-x-123[abxyz]";
    assert_eq!(
        EncryptedRoom::from(line),
        EncryptedRoom {
            name: vec![
                String::from("aaaaa"),
                String::from("bbb"),
                String::from("z"),
                String::from("y"),
                String::from("x")
            ],
            sector_id: 123,
            checksum: String::from("abxyz"),
        }
    );
}

#[test]
fn test_encrypted_room_is_real() {
    assert_eq!(
        EncryptedRoom::from("aaaaa-bbb-z-y-x-123[abxyz]").is_real(),
        true
    );
    assert_eq!(
        EncryptedRoom::from("a-b-c-d-e-f-g-h-987[abcde]").is_real(),
        true
    );
    assert_eq!(
        EncryptedRoom::from("not-a-real-room-404[oarel]").is_real(),
        true
    );
    assert_eq!(
        EncryptedRoom::from("totally-real-room-200[decoy]").is_real(),
        false
    );
}

#[test]
fn test_encrypted_room_decrypt() {
    assert_eq!(
        EncryptedRoom::from("qzmt-zixmtkozy-ivhz-343[abcde]").decrypt(),
        String::from("very encrypted name")
    );
}

#[test]
fn test_shift() {
    assert_eq!(shift(&'a', 0), 'a');
    assert_eq!(shift(&'z', 1), 'a');
    assert_eq!(shift(&'a', 27), 'b');
}
