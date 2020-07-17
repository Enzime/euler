use std::collections::HashSet;
use std::hash::Hash;

struct Sequence<T, F>
where T: Hash + Eq + Copy,
      F: Fn(usize) -> T,
{
    element_set: HashSet<T>,
    ordered_elements: Vec<T>,
    func: F,
    first_n: usize,
}

impl<T, F> Sequence<T, F>
where T: Hash + Eq + Copy,
      F: Fn(usize) -> T,
{
    fn new(func: F, first_n: usize) -> Sequence<T, F> {
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

fn main() {
    let mut triangle_nums   = Sequence::new(|n| n * (n + 1) / 2, 1);
    let mut hexagonal_nums  = Sequence::new(|n| n * (2 * n - 1), 1);
    let mut pentagonal_nums = Sequence::new(|n| n * (3 * n - 1) / 2, 1);

    let mut shared_elements: Vec<usize> = vec![];

    while shared_elements.len() < 3 {
        triangle_nums.generate_more(300);
        hexagonal_nums.generate_more(150);
        pentagonal_nums.generate_more(200);

        shared_elements = triangle_nums.element_set.intersection(&hexagonal_nums.element_set)
                                                   .cloned()
                                                   .collect::<HashSet<usize>>()
                                                   .intersection(&pentagonal_nums.element_set)
                                                   .cloned()
                                                   .collect::<Vec<_>>();
    }

    println!("{}", shared_elements[2]);
}
