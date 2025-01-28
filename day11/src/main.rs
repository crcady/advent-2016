use std::collections::HashSet;

fn main() {
    let promethium_generator = String::from("AG");
    let promethium_microchip = String::from("AM");

    let cobalt_generator = String::from("BG");
    let cobalt_microchip= String::from("BM");

    let curium_generator = String::from("CG");
    let curium_microchip = String::from("CM");

    let ruthemium_generator = String::from("DG");
    let ruthenium_micrichip = String::from("DM");

    let plutonium_generator = String::from("EG");
    let plutonium_microchip = String::from("EM");

    let elerium_generator = String::from("FG");
    let elerium_microchip = String::from("FM");

    let dilithium_generator = String::from("GG");
    let dilithium_microchip = String::from("GM");


    let gs = GameState {
        elevator: 0,
        floors: [
            vec![&promethium_generator, &promethium_microchip, &elerium_generator, &elerium_microchip, &dilithium_generator, &dilithium_microchip],
            vec![&cobalt_generator, &curium_generator, &ruthemium_generator, &plutonium_generator],
            vec![&cobalt_microchip, &curium_microchip, &ruthenium_micrichip, &plutonium_microchip],
            vec![],
        ]
    };

    let mut frontier: HashSet<GameState> = HashSet::new();
    frontier.insert(gs);

    let mut generations = 0;

    let mut seen_states: HashSet<GameState> = HashSet::new();

    'outer: loop {
        println!("Generation {} has {} candidates", generations, frontier.len());
        generations += 1;
        let mut new_frontier: HashSet<GameState> = HashSet::new();
        for current in frontier {
            for new_state in current.generate_moves() {
                if new_state.won() {
                    break 'outer;
                }
                if !seen_states.contains(&new_state) {
                    new_frontier.insert(new_state);
                }
            }
            seen_states.insert(current);
        }

        frontier = new_frontier;
    }

    println!("Found a good solution in generation {}", generations);
}
#[derive(Debug, PartialEq, Hash, Eq)]
struct GameState<'a> {
    elevator: usize,
    floors: [Vec<&'a String>; 4],
}

impl<'a> GameState<'a> {
    fn check(&self) -> bool {
        // Checks to see if this configuration causes annihilation.
        // A true value means that it's a good configuration

        for floor in self.floors.iter() {
            let mut generators: HashSet<char> = HashSet::new();
            let mut microchips: HashSet<char> = HashSet::new();

            for item in floor.iter() {
                let mut it = item.chars();
                let element = it.next().unwrap();
                let t = it.next().unwrap();

                match t {
                    'M' => microchips.insert(element),
                    'G' => generators.insert(element),
                    _ => panic!("Unexpected character found"),
                };
                
                if generators.len() == 0 {
                    continue;
                }

                for m in microchips.iter() {
                    if !generators.contains(m) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn generate_moves(&self) -> Vec<Self> {
        // For any given item on the current floor,
        // The elevator could take that item up or down a floor,
        // or take that item and one other up or down a floor

        let mut res: Vec<GameState> = Vec::new();
        for item1 in 0..self.floors[self.elevator].len() {
            if self.elevator > 0 {
                let mut new_floors: [Vec<&String>; 4] = [vec![], vec![], vec![], vec![]];
                for f in 0..4 {
                    for j in 0..self.floors[f].len() {
                        if f == self.elevator && item1 == j{
                            new_floors[f-1].push(self.floors[f][j]);
                        } else {
                            new_floors[f].push(self.floors[f][j]);
                        }
                    }
                }
                for i in 0..4 {
                    new_floors[i].sort();
                }
                let candidate = GameState {
                    elevator: self.elevator - 1,
                    floors: new_floors,
                };
                if candidate.check() {
                    res.push(candidate);
                }
            }

            if self.elevator < 3 {
                let mut new_floors: [Vec<&String>; 4] = [vec![], vec![], vec![], vec![]];
                for f in 0..4 {
                    for j in 0..self.floors[f].len() {
                        if f == self.elevator && item1 == j{
                            new_floors[f+1].push(self.floors[f][j]);
                        } else {
                            new_floors[f].push(self.floors[f][j]);
                        }
                    }
                }
                for i in 0..4 {
                    new_floors[i].sort();
                }
                let candidate = GameState {
                    elevator: self.elevator + 1,
                    floors: new_floors,
                };
                if candidate.check() {
                    res.push(candidate);
                }
            }

            for item2 in item1+1..self.floors[self.elevator].len() {

                if self.elevator > 0 {
                    let mut new_floors: [Vec<&String>; 4] = [vec![], vec![], vec![], vec![]];
                    for f in 0..4 {
                        for j in 0..self.floors[f].len() {
                            if f == self.elevator && (item1 == j || item2 == j) {
                                new_floors[f-1].push(self.floors[f][j]);
                            } else {
                                new_floors[f].push(self.floors[f][j]);
                            }
                        }
                    }
                    for i in 0..4 {
                        new_floors[i].sort();
                    }
                    let candidate = GameState {
                        elevator: self.elevator - 1,
                        floors: new_floors,
                    };
                    if candidate.check() {
                        res.push(candidate);
                    }
                }

                if self.elevator < 3 {
                    let mut new_floors: [Vec<&String>; 4] = [vec![], vec![], vec![], vec![]];
                    for f in 0..4 {
                        for j in 0..self.floors[f].len() {
                            if f == self.elevator && (item1 == j || item2 == j) {
                                new_floors[f+1].push(self.floors[f][j]);
                            } else {
                                new_floors[f].push(self.floors[f][j]);
                            }
                        }
                    }
                    for i in 0..4 {
                        new_floors[i].sort();
                    }
                    let candidate = GameState {
                        elevator: self.elevator + 1,
                        floors: new_floors,
                    };
                    if candidate.check() {
                        res.push(candidate);
                    }
                }
            }
        }

        res
    }

    fn won(&self) -> bool {
        for i in 0..3 {
            if self.floors[i].len() > 0 {
                return false
            }
        }
        true
    }
}

#[test]
fn check_check() {
    let hg = String::from("HG");
    let hm = String::from("HM");
    let lg = String::from("LG");

    let gs = GameState {
        elevator: 0,
        floors: [vec![&hg, &hm], vec![&lg], vec![], vec![]],
    };

    assert_eq!(gs.check(), true);

    let gs = GameState {
        elevator: 0,
        floors: [vec![&hm, &lg], vec![&hg], vec![], vec![]],
    };

    assert_eq!(gs.check(), false);
}

#[test]
fn check_moves() {
    let hg = String::from("HG");
    let hm = String::from("HM");
    let lg = String::from("LG");

    let gs = GameState {
        elevator: 0,
        floors: [vec![&hg, &lg, &hm], vec![], vec![], vec![]],
    };

    let moves = gs.generate_moves();

    assert_eq!(moves.len(), 4);
}