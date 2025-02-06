fn main() {
    let lucky_elf = run(3017957);
    println!("The elf with all the presents is number {}", lucky_elf);

    let lucky_elf2 = run2(3017957);
    println!("Under the advanced rules, elf {} has all the presents", lucky_elf2)
}

fn run(num_elves: usize) -> usize {
    let mut eliminated = 0;
    let mut elves = vec![true; num_elves];
    let mut idx = 0;
    let mut last_thief = 0;
    while eliminated < num_elves - 1 {
        // We are pointing either at the first elf, or the
        // last elf to have their presents stolen
        while elves[idx] == false {
            idx = (idx + 1) % num_elves;
        }
        last_thief = idx + 1;

        // We are now pointing at the next elf with presents
        // So we need to advance one more time, to steal the *next*
        // elf's presents
        idx = (idx +1) % num_elves;
        while elves[idx] == false {
            idx = (idx + 1) % num_elves;
        }
        elves[idx] = false;
        eliminated += 1;
    }

    last_thief
}

fn run2(num_elves: usize) -> usize {
    let mut eliminated = 0;
    let mut elves = vec![true; num_elves];
    let mut idx = 0;
    let mut last_thief = 0;

    let mut report_progress_at = 10;

    while eliminated < num_elves - 1 {
        if eliminated >= report_progress_at {
            println!("Eliminated {} elves", report_progress_at);
            report_progress_at *= 10;
        }
        
        // Again, we need to advance to the right to find the next thief
        while elves[idx] == false {
            idx = (idx + 1) % num_elves;
        }
        last_thief = idx + 1;

        // At this point, not counting the thief there are
        // num_elves - eliminated - 1 elves with presents

        let to_skip = match (num_elves - eliminated - 1) % 2 {
            0 => (num_elves - eliminated - 1) / 2,
            1 => (num_elves - eliminated) / 2,
            _ => unreachable!(),
        };

        let mut skipped = 0;
        let mut steal_from = idx;
        while skipped < to_skip {
            steal_from = (steal_from + 1) % num_elves;
            if elves[steal_from] {
                skipped += 1;
            }
        }

        elves[steal_from] = false;
        eliminated += 1;
        idx = (idx + 1) % num_elves;
    }

    last_thief
}

#[test]
fn test_run() {
    assert_eq!(run(5), 3);
}

#[test]
fn test_run2() {
    assert_eq!(run2(5), 2);
}