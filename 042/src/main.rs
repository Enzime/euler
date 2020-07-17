use std::collections::HashSet;

use std::fs;

fn main() {
    let word_values = fs::read_to_string("words.txt").unwrap()
                                                     .trim_matches('"')
                                                     .split("\",\"")
                                                     .map(|word| word.as_bytes()
                                                                     .iter()
                                                                     .map(|c| (c - 64) as usize)
                                                                     .sum::<usize>())
                                                     .collect::<Vec<_>>();

    let triangle_numbers = (1..).map(|n| (n + 1) * n / 2)
                                .take_while(|n| n < word_values.iter().max().unwrap())
                                .collect::<HashSet<_>>();

    println!("{}", word_values.iter().filter(|v| triangle_numbers.contains(v)).count());
}
