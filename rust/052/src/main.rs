use euler_utils::Counter;

fn main() {
    for x in 1.. {
        let x_digits = x.to_string().chars().collect::<Counter<_>>();

        let same_digits = (2..=6).map(|m| (x * m).to_string().chars().collect::<Counter<_>>())
                                 .fold(true, |acc, next| acc && next == x_digits);

        if same_digits {
            println!("{}", x);
            break;
        }
    }
}
