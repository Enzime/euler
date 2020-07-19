use std::collections::HashSet;

use euler_utils::gcd;

fn main() {
    const MAX_COORD: isize = 50;

    let mut seen = HashSet::new();

    let mut count = 0;

    for x_1 in 0..=MAX_COORD {
        for y_1 in 0..=MAX_COORD {
            if x_1 == 0 && y_1 == 0 {
                continue;
            }

            for x_2 in x_1..=MAX_COORD {
                for y_2 in 0..=MAX_COORD {
                    if x_2 == 0 && y_2 == 0 {
                        continue;
                    }

                    if x_2 == x_1 && y_2 == y_1 {
                        continue;
                    }

                    if seen.contains(&(x_2, y_2, x_1, y_1)) {
                        continue;
                    }

                    seen.insert((x_1, y_1, x_2, y_2));

                    let slopes = vec![(y_1, x_1), (y_2, x_2), (y_1 - y_2, x_1 - x_2)];

                    for a in 0..=slopes.len() {
                        for b in a+1..slopes.len() {
                            let (m_a, m_b) = (slopes[a], slopes[b]);

                            if m_a.1 == 0 {
                                if m_b.0 == 0 {
                                    count += 1;
                                }

                                continue;
                            }

                            if m_b.1 == 0 {
                                if m_a.0 == 0 {
                                    count += 1;
                                }

                                continue;
                            }

                            let s_a = (m_a.0 * m_a.1).signum();
                            let s_b = (m_b.0 * m_b.1).signum();

                            let m_a = (m_a.0.abs(), m_a.1.abs());
                            let m_b = (m_b.0.abs(), m_b.1.abs());

                            let gcd_a = gcd(m_a.0, m_a.1);
                            let gcd_b = gcd(m_b.0, m_b.1);

                            let m_a = (m_a.0 / gcd_a, m_a.1 / gcd_a);
                            let m_b = (m_b.0 / gcd_b, m_b.1 / gcd_b);

                            if s_a != s_b && m_a.0 == m_b.1 && m_b.0 == m_a.1 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
