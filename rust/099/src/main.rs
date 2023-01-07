use std::fs;

use rug::Integer;

fn main() {
    println!("{}", fs::read_to_string("base_exp.txt").unwrap()
                                                     .trim()
                                                     .split('\n')
                                                     .map(|line| line.split(",").map(|n| n.parse::<u32>()
                                                                                          .unwrap())
                                                                                .collect::<Vec<_>>())
                                                     .map(|line| Integer::from(Integer::u_pow_u(line[0], line[1])))
                                                     .enumerate()
                                                     .max_by(|(_, a), (_, b)| a.cmp(b))
                                                     .unwrap()
                                                     .0 + 1);
}
