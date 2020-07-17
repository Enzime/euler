use std::collections::HashMap;

fn main() {
    let mut hashmap: HashMap<_, String> = HashMap::new();

    hashmap.insert(18, "eighteen".to_string());
    hashmap.insert(20, "twenty".to_string());
    hashmap.insert(30, "thirty".to_string());
    hashmap.insert(40, "forty".to_string());
    hashmap.insert(50, "fifty".to_string());
    hashmap.insert(80, "eighty".to_string());
    hashmap.insert(1000, "one thousand".to_string());

    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen"];

    for (n, word) in words.iter().enumerate() {
        hashmap.insert(n, word.to_string());
    }

    for d in 6..=9 {
        if d == 8 {
            continue;
        }

        // 16 -> 19
        hashmap.insert(10 + d, format!("{}teen", words[d]));

        // 60 -> 90
        hashmap.insert(d * 10, format!("{}ty", words[d]));
    }

    for d in 1..=9 {
        // 100 -> 900
        hashmap.insert(d * 100, format!("{} hundred", words[d]));

        for tens in 2..=9 {
            // 21 -> 99
            hashmap.insert(tens * 10 + d, format!("{} {}", hashmap.get(&(tens * 10)).unwrap(), words[d]));
        }
    }

    for n in 101..=999 {
        if n % 100 == 0 {
            continue;
        }

        hashmap.insert(n, format!("{} and {}", hashmap.get(&(n - n % 100)).unwrap(), hashmap.get(&(n % 100)).unwrap()));
    }

    for n in 0..=1000 {
        println!("{} {}", n, match hashmap.get(&n) {
            Some(x) => x,
            None => "None",
        });
    }

    println!("{}", (1..=1000).map(|n| hashmap.get(&n).unwrap().chars().filter(|c| *c != ' ').count()).sum::<usize>());
}
