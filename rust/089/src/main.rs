use std::collections::HashMap;
use std::fs;
use std::iter;

fn parse_valid_roman_numeral(value: &HashMap<char, usize>, roman_numeral: &str) -> usize {
    let roman_numeral = roman_numeral.chars().collect::<Vec<_>>();
    let mut number = 0;
    let mut i = 0;

    while i < roman_numeral.len() {
        let a = roman_numeral[i];

        if i == roman_numeral.len() - 1 {
            number += value[&a];
            i += 1;
            continue;
        }

        let b = roman_numeral[i + 1];

        if value[&a] >= value[&b] {
            number += value[&a];
            i += 1;
            continue;
        }

        number += value[&b] - value[&a];
        i += 2;
    }

    number
}

fn generate_minimal_roman_numeral(mut n: usize) -> String {
    let mut result = vec![];

    if n >= 1_000 {
        result.extend(iter::repeat("M").take(n / 1_000));
        n %= 1_000;
    }

    let mut i = 1;

    let mut temp = vec![];

    while n > 0 {
        let column_value = n % 10;

        let (one, five, ten) = match i {
            1 => ("I", "V", "X"),
            2 => ("X", "L", "C"),
            3 => ("C", "D", "M"),
            _ => unreachable!(),
        };

        temp.push(match column_value {
            0 => vec![],
            1 | 2 | 3 => iter::repeat(one).take(column_value).collect(),
            4 => vec![one, five],
            5 | 6 | 7 | 8 => {
                let mut extension = vec![five];
                extension.extend(iter::repeat(one).take(column_value - 5));
                extension
            },
            9 => vec![one, ten],
            _ => unreachable!(),
        });

        n /= 10;
        i += 1;
    }

    for v in temp.into_iter().rev() {
        result.extend(v);
    }

    result.join("")
}

fn main() {
    let value: HashMap<_, _> = vec![
        ('I', 1), ('V', 5), ('X', 10), ('L', 50), ('C', 100), ('D', 500), ('M', 1_000)
    ].into_iter().collect();

    assert_eq!(parse_valid_roman_numeral(&value, "I"), 1);
    assert_eq!(parse_valid_roman_numeral(&value, "IV"), 4);
    assert_eq!(parse_valid_roman_numeral(&value, "IX"), 9);
    assert_eq!(parse_valid_roman_numeral(&value, "XL"), 40);
    assert_eq!(parse_valid_roman_numeral(&value, "XC"), 90);
    assert_eq!(parse_valid_roman_numeral(&value, "CD"), 400);
    assert_eq!(parse_valid_roman_numeral(&value, "CM"), 900);
    assert_eq!(parse_valid_roman_numeral(&value, "CMXLVIII"), 948);
    assert_eq!(parse_valid_roman_numeral(&value, "CMXLIX"), 949);
    assert_eq!(generate_minimal_roman_numeral(1_000), "M");
    assert_eq!(generate_minimal_roman_numeral(2_000), "MM");
    assert_eq!(generate_minimal_roman_numeral(2_500), "MMD");
    assert_eq!(generate_minimal_roman_numeral(2_550), "MMDL");
    assert_eq!(generate_minimal_roman_numeral(8), "VIII");
    assert_eq!(generate_minimal_roman_numeral(9), "IX");

    for i in 1..=1_000 {
        assert_eq!(parse_valid_roman_numeral(&value, &generate_minimal_roman_numeral(i)), i);
    }

    let roman_numerals = fs::read_to_string("roman.txt").unwrap();
    let roman_numerals = roman_numerals.trim().split("\n").collect::<Vec<_>>();

    let mut characters_saved = 0;

    for roman_numeral in roman_numerals {
        let number = parse_valid_roman_numeral(&value, roman_numeral);
        let minimal = generate_minimal_roman_numeral(number);

        characters_saved += roman_numeral.len() - minimal.len();
    }

    println!("{}", characters_saved);
}
