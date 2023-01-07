use std::cmp;
use std::fs;

fn main() {
    let matrix = fs::read_to_string("matrix.txt").unwrap()
                                                 .trim()
                                                 .split("\n")
                                                 .map(|line| line.split(",")
                                                                 .map(|cell| cell.parse::<usize>().unwrap())
                                                                 .collect::<Vec<_>>())
                                                 .collect::<Vec<_>>();

    let M = matrix.len();
    let N = matrix[0].len();

    let mut sums = vec![vec![0; N]; M];

    for m in 0..M {
        for n in 0..N {
            let mut min = std::usize::MAX;

            if m > 0 {
                min = cmp::min(sums[m-1][n], min);
            }

            if n > 0 {
                min = cmp::min(sums[m][n-1], min);
            }

            if min == std::usize::MAX {
                min = 0;
            }

            min += matrix[m][n];
            sums[m][n] = min;
        }
    }
    
    println!("{:?}", sums);
}
