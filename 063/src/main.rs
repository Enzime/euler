fn main() {
    let mut nth_powers_found = 0;

    for n in 1.. {
        let start = 10u128.pow(n - 1);
        let end = 10u128.pow(n) - 1;

        let mut nth_power_found = false;

        for i in 1u128.. {
            let number = i.pow(n);

            if number > end {
                break;
            }

            if number >= start {
                nth_power_found = true;
                nth_powers_found += 1;
            }
        }

        if !nth_power_found {
            break;
        }
    }

    println!("{}", nth_powers_found);
}
