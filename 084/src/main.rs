use euler_utils::Counter;

use rand::prelude::*;
use rand::distributions::Uniform;

fn main() {
    const GO: usize = 0;
    const JAIL: usize = 10;
    const C1: usize = 11;
    const E3: usize = 24;
    const H2: usize = 39;
    const R1: usize = 5;

    let cells = vec!["GO", "A1", "CC1", "A2", "T1", "R1", "B1", "CH1", "B2", "B3", "JAIL", "C1", "U1", "C2", "C3", "R2", "D1", "CC2", "D2", "D3", "FP", "E1", "CH2", "E2", "E3", "R3", "F1", "F2", "U2", "F3", "G2J", "G1", "G2", "CC3", "G3", "R4", "CH3", "H1", "T2", "H2"];

    let mut rng = thread_rng();

    let dice_range = Uniform::new_inclusive(1, 6);

    let mut community_chest = (1..=16).collect::<Vec<_>>();
    community_chest.shuffle(&mut rng);

    let mut chance = (1..=16).collect::<Vec<_>>();
    chance.shuffle(&mut rng);

    let mut cell_counts = Counter::new();

    for _ in 0..1_000 {
        let mut current_cell = 0;

        for _ in 0..1_000 {
            let dice_roll = rng.sample(dice_range) + rng.sample(dice_range);

            current_cell = (current_cell + dice_roll) % 40;

            if cells[current_cell].starts_with("CC") {
                let card = community_chest.remove(0);

                current_cell = match card {
                    1 => GO,
                    2 => JAIL,
                    _ => current_cell,
                };

                community_chest.push(card);
            }

            if cells[current_cell].starts_with("CH") {
                let card = chance.remove(0);

                current_cell = match card {
                    1 => GO,
                    2 => JAIL,
                    3 => C1,
                    4 => E3,
                    5 => H2,
                    6 => R1,
                    7 | 8 => ((current_cell + 1..).take_while(|c| !cells[c % 40].starts_with("R")).last().unwrap() + 1) % 40,
                    9 => ((current_cell + 1..).take_while(|c| !cells[c % 40].starts_with("U")).last().unwrap() + 1) % 40,
                    10 => if current_cell >= 3 { current_cell - 3 } else { 37 + current_cell },
                    _ => current_cell,
                };

                chance.push(card);
            }

            if cells[current_cell] == "G2J" {
                current_cell = JAIL;
            }

            cell_counts[cells[current_cell]] += 1;
        }
    }

    let cell_counts = cell_counts.sorted();

    println!("{:?}", cell_counts);

    println!("{}", cell_counts.iter().map(|(cell, _)| cells.iter().position(|v| v == cell).unwrap()).take(3).map(|cell| format!("{:02}", cell)).collect::<String>());
}
