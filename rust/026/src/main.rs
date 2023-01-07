use std::collections::HashMap;

fn calculate_recurring_cycle(d: usize) -> usize {
    let mut long_division_graph = HashMap::new();
    let mut n = 1;

    loop {
        let new_n = (n % d) * 10;

        if new_n == 0 {
            return 0;
        }

        long_division_graph.insert(n, new_n);

        n = new_n;

        if long_division_graph.contains_key(&new_n) {
            break;
        }
    }

    let mut cycle_length = 1;
    let mut cur = long_division_graph[&n];

    while cur != n {
        cycle_length += 1;

        cur = long_division_graph[&cur];
    }

    cycle_length
}

fn main() {
    println!("{}", (2..1000).map(|d| (calculate_recurring_cycle(d), d)).max().unwrap().1);
}
