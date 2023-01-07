use std::cmp;
use std::fs;

fn main() {
    let triangle = fs::read_to_string("triangle.txt").unwrap();

    let triangle = triangle.trim();

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
