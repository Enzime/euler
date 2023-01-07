use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;

struct Sequence<T>
where
    T: Hash + Eq + Copy,
{
    element_set: HashSet<T>,
    ordered_elements: Vec<T>,
    func: Box<dyn Fn(usize) -> T>,
    first_n: usize,
}

impl<T> Sequence<T>
where
    T: Hash + Eq + Copy,
{
    fn new(func: Box<dyn Fn(usize) -> T>, first_n: usize) -> Sequence<T> {
        Sequence { element_set: HashSet::new(), ordered_elements: vec![], func: func, first_n: first_n }
    }

    fn len(&self) -> usize {
        self.ordered_elements.len()
    }

    fn generate_more(&mut self, n: usize) {
        for i in self.first_n + self.len()..self.first_n + self.len() + n {
            let s_i = (self.func)(i);

            self.element_set.insert(s_i);
            self.ordered_elements.push(s_i);
        }
    }
}

impl<T> Index<usize> for Sequence<T>
where
    T: Hash + Eq + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.ordered_elements[index]
    }
}

fn solve<'a>(sequences: &mut HashMap<&'a str, Vec<usize>>, option: &'a str, solution: &mut Vec<usize>) -> bool {
    let current = sequences.remove(option).unwrap();

    for n in &current {
        if solution.len() > 0 {
            if solution.last().unwrap() % 100 != n / 100 {
                continue;
            }
        }

        solution.push(*n);

        if solution.len() == 6 {
            if solution[5] % 100 == solution[0] / 100 {
                return true;
            }
        }

        let options = sequences.iter().map(|(k, _)| k).cloned().collect::<Vec<_>>();

        for sequence_to_try in options {
            if solve(sequences, sequence_to_try, solution) {
                return true;
            }
        }

        solution.pop();
    }

    sequences.insert(option, current);
    return false;
}

fn main() {
    let mut sequences = HashMap::new();
    sequences.insert("triangle",   Sequence::new(Box::new(|n| n*(n+1)/2)  , 1));
    sequences.insert("square",     Sequence::new(Box::new(|n| n*n)        , 1));
    sequences.insert("pentagonal", Sequence::new(Box::new(|n| n*(3*n-1)/2), 1));
    sequences.insert("hexagonal",  Sequence::new(Box::new(|n| n*(2*n-1))  , 1));
    sequences.insert("heptagonal", Sequence::new(Box::new(|n| n*(5*n-3)/2), 1));
    sequences.insert("octagonal",  Sequence::new(Box::new(|n| n*(3*n-2))  , 1));

    let mut four_digit_sequence_nums = HashMap::new();

    for (name, mut sequence) in sequences {
        sequence.generate_more(100);

        while sequence[sequence.len() - 1] < 10_000 {
            sequence.generate_more(100);
        }

        four_digit_sequence_nums.insert(name, sequence.ordered_elements.iter()
                                                                       .cloned()
                                                                       .filter(|i| *i >= 1_000 && *i < 10_000)
                                                                       .collect::<Vec<_>>());
    }

    let mut solution = vec![];
    solve(&mut four_digit_sequence_nums, "triangle", &mut solution);
    println!("{}", solution.iter().sum::<usize>());
}
