use rug::Float;

fn main() {
    let mut sum = 0;

    for n in 2..100 {
        let sqrt_n = Float::with_val(360, n).sqrt();

        if sqrt_n.is_integer() {
            continue;
        }

        let float_string = sqrt_n.to_string_radix(10, Some(103));

        let digital_sum = float_string.chars()
                                      .filter(|&x| x != '.')
                                      .collect::<Vec<_>>()[0..100]
                                      .iter()
                                      .map(|c| c.to_digit(10).unwrap())
                                      .sum::<u32>();

        sum += digital_sum;
    }

    println!("{}", sum);
}
