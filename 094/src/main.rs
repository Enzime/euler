use rug::{Assign, Integer};

fn main() {
    let mut perimeter_sum = 0;

    let mut area = Integer::new();
    let mut rem = Default::default();

    'outer: for c in 3i64.. {
        let a = c - 1;

        for b in a..=c {
            let perimeter = a + b + c;

            if perimeter > 1_000_000_000 {
                if b == a {
                    break 'outer;
                }

                break;
            }

            area.assign(perimeter);
            area *= b + c - a;
            area *= a - b + c;
            area *= a + b - c;

            area.sqrt_rem_mut(&mut rem);

            if rem == 0 && area.to_u64().unwrap() % 4 == 0 {
                perimeter_sum += perimeter;
            }
        }
    }

    println!("{}", perimeter_sum);
}
