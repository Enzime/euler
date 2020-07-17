use rug::Integer;

fn main() {
    let mut expansions = 0;

    let mut numerator = Integer::from(3);
    let mut denominator = Integer::from(2);

    for i in 2..=1_000 {
        denominator += &numerator;
        numerator = &denominator + Integer::from(&denominator - &numerator);

        if numerator.to_string().len() > denominator.to_string().len() {
            expansions += 1;
        } 
    }

    println!("{}", expansions);
}
