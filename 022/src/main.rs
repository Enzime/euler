use std::fs;

fn main() {
    let names = fs::read_to_string("names.txt").unwrap();

    let mut names = names.split(",")
                     .collect::<Vec<_>>();

    names.sort();

    let names = names.iter()
                     .map(|name| name.trim_matches('"')
                                     .chars()
                                     .map(|l| l as u32 - 64)
                                     .sum::<u32>())
                     .enumerate()
                     .map(|(pos, name)| (pos + 1) * name as usize)
                     .collect::<Vec<_>>();

    println!("{:?}", names);
    println!("{}", names.iter().sum::<usize>());
}
