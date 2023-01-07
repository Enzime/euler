use euler_utils::nth_permutation;

fn main() {
    let mut operators: Vec<Box<dyn Fn(f64, f64) -> f64>> = vec![];

    operators.push(Box::new(|x: f64, y: f64| -> f64 { x + y }));
    operators.push(Box::new(|x: f64, y: f64| -> f64 { x - y }));
    operators.push(Box::new(|x: f64, y: f64| -> f64 { x * y }));
    operators.push(Box::new(|x: f64, y: f64| -> f64 { x / y }));

    let mut best_found = (0, Default::default());

    for a in 1..=9 {
        for b in a+1..=9 {
            for c in b+1..=9 {
                for d in c+1..=9 {
                    let mut constructable = [false; 10_000];

                    for i in 0..24 {
                        let current = nth_permutation(vec![a as f64, b as f64, c as f64, d as f64], i);
                        let a = current[0];
                        let b = current[1];
                        let c = current[2];
                        let d = current[3];

                        for o1 in &operators {
                            for o2 in &operators {
                                for o3 in &operators {
                                    let mut constructable_numbers = vec![];

                                    // ((a . b) . c) . d
                                    constructable_numbers.push(o3(o2(o1(a, b), c), d));

                                    // (a . (b . c)) . d
                                    constructable_numbers.push(o3(o1(a, o2(b, c)), d));

                                    // a . ((b . c) . d)
                                    constructable_numbers.push(o1(a, o3(o2(b, c), d)));

                                    // a . (b . (c . d))
                                    constructable_numbers.push(o1(a, o2(b, o3(c, d))));

                                    // (a . b) . (c . d)
                                    constructable_numbers.push(o2(o1(a, b), o3(c, d)));

                                    for n in constructable_numbers {
                                        if n.is_finite() && n.fract() <= f64::EPSILON && n > 0.0 {
                                            constructable[n as usize] = true;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let first_n_constructable = (1..10_000).take_while(|&n| constructable[n]).count();

                    if first_n_constructable > best_found.0 {
                        best_found = (first_n_constructable, format!("{}{}{}{}", a, b, c, d));
                    }
                }
            }
        }
    }

    println!("{}", best_found.1);
}
