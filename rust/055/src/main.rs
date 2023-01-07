fn main() {
    let mut lychrel_numbers = vec![];

    'next_n: for n in 1..10_000 {
        let mut cur = n;

        for i in 0..50 {
            cur = cur + cur.to_string().chars().rev().collect::<String>().parse::<u128>().unwrap();

            if cur == cur.to_string().chars().rev().collect::<String>().parse::<u128>().unwrap() {
                continue 'next_n;
            }
        }

        lychrel_numbers.push(n);
    }

    println!("{:?}", lychrel_numbers);
    println!("{}", lychrel_numbers.len());
}
