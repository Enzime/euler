use std::collections::HashMap;

fn main() {
    let mut chain_lengths = HashMap::new();

    // base case
    chain_lengths.insert(1, 1);

    for n in 2u64..1_000_000u64 {
        if chain_lengths.contains_key(&n) {
            continue;
        }

        let mut cur = n;
        let mut seq_stack = vec![];

        while chain_lengths.get(&cur) == None {
            seq_stack.push(cur);

            if cur % 2 == 0 {
                cur = cur / 2;
            } else {
                cur = 3 * cur + 1;
            }
        }

        let base_chain_length = *chain_lengths.get(&cur).unwrap();

        for (pos, i) in seq_stack.iter().enumerate() {
            chain_lengths.insert(*i, base_chain_length + (seq_stack.len() - pos));
        }
    }

    println!("{:?}", (1..1_000_000).map(|k| (chain_lengths.get(&k).unwrap(), k)).max().unwrap().1);
}
