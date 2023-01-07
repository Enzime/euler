fn main() {
    const N: usize = 100;
    let mut unique_sums_using = [[1; N + 1]; N + 1];

    for i in 2..=N {
        for j in 0..=N {
            if j < i {
                unique_sums_using[i][j] = unique_sums_using[i-1][j];
            } else {
                unique_sums_using[i][j] = unique_sums_using[i-1][j] + unique_sums_using[i][j-i];
            }
        }
    }

    for i in 1..=N {
        println!("{:?}", unique_sums_using[i].to_vec());
    }

    println!("{}", unique_sums_using[N][N] - 1);
}
