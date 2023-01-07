fn main() {
    let mut indices_to_find = (0..=6).map(|p| 10usize.pow(p)).collect::<Vec<_>>();

    let mut i = 0;
    let mut d_n = 0;

    let mut values = vec![];

    while indices_to_find.len() > 0 {
        if d_n >= indices_to_find[0] {
            let index = indices_to_find.remove(0);

            let mut cur_i = i;

            for _ in 0..d_n - index {
                cur_i /= 10;
            }

            values.push(cur_i % 10);
            // println!("d_{}: {}", index, cur_i % 10);
        }

        i += 1;
        d_n += i.to_string().len();
    }

    println!("{}", values.iter().product::<usize>());
}
