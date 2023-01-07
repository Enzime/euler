use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let words = fs::read_to_string("words.txt").unwrap();
    let words = words.trim_matches('"').split("\",\"").collect::<Vec<_>>();

    let mut anagrams = HashMap::new();

    for word in words {
        let mut key = word.chars().collect::<Vec<_>>();
        key.sort();
        let key = key.into_iter().collect::<String>();

        anagrams.entry(key).or_insert(vec![]).push(word);
    }

    anagrams = anagrams.into_iter()
                       .filter(|(_, v)| v.len() > 1)
                       .collect();

    let digits = anagrams.keys().map(|k| k.len()).max().unwrap() / 2 + 1;

    let mut square_patterns = HashMap::new();

    for n in 0..10usize.pow(digits as u32) {
        let n_squared = (n * n).to_string();
        let mut pattern_mapping = HashMap::new();
        let mut pattern = vec![];

        for c in n_squared.chars() {
            if !pattern_mapping.contains_key(&c) {
                pattern_mapping.insert(c, pattern_mapping.len());
            }

            pattern.push(pattern_mapping[&c]);
        }

        square_patterns.entry(pattern).or_insert(vec![]).push(n_squared);
    }

    let mut largest_square = 0;
    
    for (_, words) in &anagrams {
        for (i, a) in words.iter().enumerate() {
            let a = a.chars().collect::<Vec<_>>();
            let mut pattern_mapping = HashMap::new();
            let mut pattern = vec![];

            for c in &a {
                if !pattern_mapping.contains_key(&c) {
                    pattern_mapping.insert(c, pattern_mapping.len());
                }

                pattern.push(pattern_mapping[&c]);
            }

            for (j, b) in words.iter().enumerate() {
                if j <= i {
                    continue;
                }

                'next_square: for square in &square_patterns[&pattern] {
                    let mut letter_mapping = HashMap::new();

                    for (k, digit) in square.chars().enumerate() {
                        let digit = digit.to_digit(10).unwrap() as usize;

                        if *letter_mapping.entry(a[k]).or_insert(digit) != digit {
                            unreachable!();
                        }
                    }

                    let mut b_square = 0;

                    for (x, c) in b.chars().enumerate() {
                        if x == 0 && letter_mapping[&c] == 0 {
                            continue 'next_square;
                        }

                        b_square += letter_mapping[&c];

                        if x + 1 < b.len() {
                            b_square *= 10;
                        }
                    }

                    let sqrt = (b_square as f64).sqrt() as usize;

                    if sqrt * sqrt != b_square {
                        continue;
                    }

                    largest_square = cmp::max(b_square, largest_square);
                }
            }
        }
    }

    println!("{}", largest_square);
}
