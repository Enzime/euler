use std::cmp;
use std::collections::HashMap;

use rug::Integer;

fn main() {
    let mut cubic_permutations = HashMap::new();

    let mut final_i_for = HashMap::new();
    let mut is_finalized = HashMap::new(); 

    let mut final_k = Integer::from(0);

    for i in 345.. {
        let cube = Integer::from(Integer::u_pow_u(i, 3));

        let key = {
            let mut digits = cube.to_string().chars().collect::<Vec<_>>();
            digits.sort();
            digits.into_iter().collect::<String>()
        };

        if cubic_permutations.entry(key.clone()).or_insert(vec![]).len() == 0 {
            let final_i = {
                let rev_key = Integer::from(Integer::parse(key.chars().rev().collect::<String>()).unwrap());
                let (cube_root, rem) = rev_key.root_rem(Integer::new(), 3);

                if rem == 0 { cube_root } else { cube_root + 1 }
            };

            final_i_for.insert(key.clone(), final_i.clone());
            is_finalized.insert(key.clone(), false);
        }

        cubic_permutations.get_mut(&key).unwrap().push(cube.clone());

        for (k, v) in is_finalized.iter_mut() {
            if *v {
                continue;
            }

            if final_i_for[k] < i {
                *v = true;
            }
        }

        if final_k == i {
            let mut answer = cube.clone();

            for (_, perms) in &cubic_permutations {
                if perms.len() == 5 {
                    answer = cmp::min(answer, perms[0].clone());
                }
            }

            println!("{}", answer);
            break;
        }

        if cubic_permutations[&key].len() == 5 && final_k == 0 {
            for (k, perms) in &cubic_permutations {
                if perms[0] < cubic_permutations[&key][0] && !is_finalized[k] {
                    final_k = cmp::max(final_k, final_i_for[k].clone());
                }
            }
        }
    }
}
