fn main() {
    const N: u128 = 2;
    let mut x = vec![1, 3];
    let mut y = vec![0, 2];
    let mut k = 1;

    let mut delta_x = 4;
    let mut delta_y = 3;

    while delta_x < 1_000_000_000_000u128 {
        x.push(x[1]*x[k] + N*y[1]*y[k]);
        y.push(x[1]*y[k] + y[1]*x[k]);

        delta_x += x[k+1];
        delta_y += y[k+1];

        k += 1;
    }

    println!("{}", delta_y);
}
