use rug::Integer;

fn main() {
    let n = Integer::from(Integer::u_pow_u(2, 1000));

    println!("{}", n.to_string().chars().map(|d| d.to_digit(10).unwrap()).sum::<u32>());
}
