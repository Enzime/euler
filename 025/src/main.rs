use rug::Integer;

fn main() {
    println!("{}", (0..).map(Integer::fibonacci).map(Integer::from).map(|f_n| f_n.to_string().len()).take_while(|l| *l < 1_000).count());
}
