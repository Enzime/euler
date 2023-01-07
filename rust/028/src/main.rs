fn f(n: usize) -> usize {
    let n = n / 2;

    1 + (8 * n*(n+1)*(2*n + 1)) / 3 + 2*n*n + 6*n
}

fn main() {
    assert_eq!(f(1), 1);
    assert_eq!(f(3), 25);
    assert_eq!(f(5), 101);

    println!("{}", f(1001));
}
