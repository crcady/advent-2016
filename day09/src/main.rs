use std::fs;
use regex::Regex;

fn main() {
    let compressed_content = fs::read_to_string("input.txt").unwrap();
    let decompressed_content = decompress(&compressed_content);
    println!("The file decompressed to {} bytes", decompressed_content.len());
    println!("Whoa! The file actually decompressed to {} bytes", expanded_size(&compressed_content));
}

fn decompress(compressed: &str) -> String {
    let re = Regex::new(r"\([0-9]+x[0-9]+\)").unwrap();
    let mut current: usize = 0;
    let mut decompressed = String::new();
    while current < compressed.len() {
        if let Some(m) =  re.find(&compressed[current..]) {
            // The offsets in the match are into the slice searched, not the whole compressed string
            let start = current + m.start();
            let end = current + m.end();

            // Push the characters since the last match, if any
            decompressed.push_str(&compressed[current..start]);

            // Parse the marker
            let args: Vec<_> = compressed[start+1..end-1].split("x").collect();
            let char_count: usize = args[0].parse().unwrap();
            let num_reps: usize = args[1].parse().unwrap();

            // Write the output to the decompressed string
            for _ in 0..num_reps {
                decompressed.push_str(&compressed[end..end+char_count]);
            }

            // Update where we are
            current = end + char_count

        } else {
            // If we arrive here, there are no more matches, but we have chars remaining in the compressed slice
            decompressed.push_str(&compressed[current..]);
            current = compressed.len();
        }
    }
    decompressed
}

fn expanded_size(compressed: &str) -> usize {
    let re = Regex::new(r"\([0-9]+x[0-9]+\)").unwrap();
    let mut num_bytes: usize = 0;
    let mut current: usize = 0;

    while current < compressed.len() {
        if let Some(m) = re.find(&compressed[current..]) {
            // Recompute offsets
            let start = current + m.start();
            let end = current + m.end();

            // Handle any intervening text before the marker
            num_bytes += start - current;

            // Parse the marker
            let args: Vec<_> = compressed[start+1..end-1].split("x").collect();
            let char_count: usize = args[0].parse().unwrap();
            let num_reps: usize = args[1].parse().unwrap();

            // Recursively handle the compressed text
            num_bytes += num_reps * expanded_size(&compressed[end..end+char_count]);

            // Update where we are
            current = end + char_count;

        } else {
            // No markers ramaining
            num_bytes += compressed.len() - current;
            current = compressed.len();
        }
    }
    num_bytes
}

#[test]
fn test_decompress() {
    assert_eq!(&decompress("ADVENT"), "ADVENT");
    assert_eq!(&decompress("A(1x5)BC"), "ABBBBBC");
    assert_eq!(&decompress("(3x3)XYZ"), "XYZXYZXYZ");
    assert_eq!(&decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
    assert_eq!(&decompress("(6x1)(1x3)A"), "(1x3)A");
    assert_eq!(&decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
}

#[test]
fn test_expanded_size() {
    assert_eq!(expanded_size("ADVENT"), 6);
    assert_eq!(expanded_size("A(1x5)BC"), 7);
    assert_eq!(expanded_size("(3x3)XYZ"), 9);
    assert_eq!(expanded_size("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(expanded_size("X(8x2)(3x3)ABCY"), 20);
    assert_eq!(expanded_size("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(expanded_size("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
}
