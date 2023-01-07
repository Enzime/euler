fn main() {
    const AMOUNT_TO_MAKE: usize = 200;
    const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

    let mut dp = [[0; AMOUNT_TO_MAKE + 1]; COINS.len()];

    for j in 1..=AMOUNT_TO_MAKE {
        dp[0][j] = 1;
    }

    for i in 1..COINS.len() {
        for j in 0..=AMOUNT_TO_MAKE {
            dp[i][j] = dp[i-1][j];

            if j == COINS[i] {
                dp[i][j] += 1;
            } else if j > COINS[i] {
                dp[i][j] += dp[i][j - COINS[i]];
            }
        }
    }

    println!("{:?}", &dp[2][0..]);
    println!("{}", dp[COINS.len() - 1][200]);
}
