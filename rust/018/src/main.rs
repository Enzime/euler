use std::cmp;

fn main() {
    let triangle = "75                                        \n\
                    95 64                                     \n\
                    17 47 82                                  \n\
                    18 35 87 10                               \n\
                    20 04 82 47 65                            \n\
                    19 01 23 75 03 34                         \n\
                    88 02 77 73 07 63 67                      \n\
                    99 65 04 28 06 16 70 92                   \n\
                    41 41 26 56 83 40 80 70 33                \n\
                    41 48 72 33 47 32 37 16 94 29             \n\
                    53 71 44 65 25 43 91 52 97 51 14          \n\
                    70 11 33 28 77 73 17 78 39 68 17 57       \n\
                    91 71 52 38 17 14 91 43 58 50 27 29 48    \n\
                    63 66 04 68 89 53 67 30 73 16 69 87 40 31 \n\
                    04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

    let triangle = "3\n\
                   7 4\n\
                  2 4 6\n\
                 8 5 9 3";

    let triangle = triangle.split("\n")
                           .map(|line| line.trim()
                                           .split(" ")
                                           .map(|n| n.parse::<u32>().unwrap())
                                           .collect::<Vec<_>>())
                           .collect::<Vec<_>>();

    let mut cumulative_sums: Vec<Vec<u32>> = vec![triangle.last().unwrap().to_vec()];

    for level in (0..triangle.len() - 1).rev() {
        let mut current_sums = vec![];

        for i in 0..triangle[level].len() {
            let level_below = (triangle.len() - 2) - level;

            current_sums.push(triangle[level][i]
                              + cmp::max(cumulative_sums[level_below][i],
                                         cumulative_sums[level_below][i + 1]));
        }

        cumulative_sums.push(current_sums);
    }

    println!("{}", cumulative_sums.last().unwrap()[0]);
}
