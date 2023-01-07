fn main() {
    let mut solutions = 0;

    for size in 1.. {
        for l in 1..=size {
            for w in l..=size {
                let mut h_start = w;

                if l != size {
                    h_start = size;
                }

                for h in h_start..=size {
                    let mut candidates = vec![];

                    let dimensions = vec![l, w, h];

                    for parameterized_dimension in 0..3 {
                        let (f1, f2) = match parameterized_dimension {
                            0 => (w, h),
                            1 => (h, l),
                            2 => (l, w),
                            _ => panic!(),
                        };

                        let p = dimensions[parameterized_dimension];

                        let sqrt = ((p*p + (f1+f2)*(f1+f2)) as f64).sqrt();

                        candidates.push(sqrt);
                    }

                    if candidates.iter()
                                 .min_by(|x, y| x.partial_cmp(y).unwrap())
                                 .unwrap()
                                 .fract() <= 1e-10 {
                        solutions += 1;
                    }
                }
            }
        }

        println!("{} {}", size, solutions);

        if solutions > 1_000_000 {
            break;
        }
    }
}
