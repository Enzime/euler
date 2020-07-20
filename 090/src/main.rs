fn choices<T: Copy + PartialOrd>(items: Vec<T>, r: usize) -> Vec<Vec<T>> {
    if r == 1 {
        return items.into_iter().map(|item| vec![item]).collect();
    }

    let mut output = vec![];

    for (i, &item) in items.iter().enumerate() {
        for mut choice in choices([&items[..i], &items[i+1..]].concat(), r-1) {
            if item < choice[0] {
                choice.insert(0, item);
                output.push(choice);
            }
        }
    }

    output
}

fn main() {
    assert_eq!(choices(vec![0, 1, 2], 2), vec![vec![0, 1], vec![0, 2], vec![1, 2]]);

    let possible_cubes = choices((0..10).collect(), 6);

    let mut distinct_arrangements = 0;

    for (i, d1) in possible_cubes.iter().enumerate() {
        for (j, d2) in possible_cubes.iter().enumerate() {
            if j < i {
                continue;
            }

            let mut number_formable = vec![false; 100];

            for &a in d1 {
                let mut a_possible = vec![a];

                if a == 6 {
                    a_possible.push(9);
                } else if a == 9 {
                    a_possible.push(6);
                }

                for &b in d2 {
                    let mut b_possible = vec![b];

                    if b == 6 {
                        b_possible.push(9);
                    } else if b == 9 {
                        b_possible.push(6);
                    }

                    for a in &a_possible {
                        for b in &b_possible {
                            number_formable[a*10 + b] = true;
                            number_formable[b*10 + a] = true;
                        }
                    }
                }
            }

            if (1..10).all(|i| number_formable[i*i]) {
                distinct_arrangements += 1;
            }
        }
    }

    println!("{}", distinct_arrangements);
}
