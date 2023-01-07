use std::fmt::Debug;

fn nth_permutation<T: Copy + Debug>(items: Vec<T>, n: usize) -> Vec<T> {
    if items.len() == 1 {
        return vec![items[0]];
    }
  
    let n_minus_one_factorial = (1..=items.len() - 1).product::<usize>();

    let index = (n as f64 / n_minus_one_factorial as f64) as usize;

    let mut result = vec![items[index]];

    result.extend(nth_permutation([&items[..index], &items[index + 1..]].concat(), n % n_minus_one_factorial));

    result
}

fn main() {
    assert_eq!(nth_permutation(vec![0, 1, 2], 0), [0, 1, 2]);
    assert_eq!(nth_permutation(vec![0, 1, 2], 1), [0, 2, 1]);
    assert_eq!(nth_permutation(vec![0, 1, 2], 2), [1, 0, 2]);
    assert_eq!(nth_permutation(vec![0, 1, 2], 3), [1, 2, 0]);
    assert_eq!(nth_permutation(vec![0, 1, 2], 4), [2, 0, 1]);
    assert_eq!(nth_permutation(vec![0, 1, 2], 5), [2, 1, 0]);

    assert_eq!(nth_permutation(vec![0, 1, 2, 3], 5), [0, 3, 2, 1]);
    assert_eq!(nth_permutation(vec![0, 1, 2, 3], 6), [1, 0, 2, 3]);

    let one_millionth_perm = nth_permutation((0..=9).collect(), 999_999);
    println!("{}", one_millionth_perm.iter().map(|d| d.to_string()).collect::<String>());
}
