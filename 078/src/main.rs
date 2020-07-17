fn main() {
    const N: usize = 100_000;

    let mut partitions = vec![1; N + 1];

    'outer: for i in 2..=N {
        for j in 2..=N {
            if j >= i {
                partitions[j] += partitions[j-i];
                partitions[j] %= 1_000_000;
            }

            if j < i && partitions[j] == 0 {
                println!("{}", j);
                break 'outer;
            }
        }
    }
}
