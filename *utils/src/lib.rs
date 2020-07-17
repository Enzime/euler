use core::hash::Hash;
use core::iter::FromIterator;
use core::ops::{Add, Index, IndexMut, Sub};
use std::collections::{hash_map, HashMap};

use num_traits::{FromPrimitive, Zero};
use num_integer::Integer;

#[derive(Clone, Debug, PartialEq)]
pub struct Counter<T>
where
    T: Eq + Hash,
{
    map: HashMap<T, usize>,
}

impl<T> Counter<T> 
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn sorted(self) -> Vec<(T, usize)> {
        let mut vec = self.map.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.1.cmp(&a.1));
        vec
    }

    pub fn remove(&mut self, key: T) {
        self.map.remove(&key);
    }
}

impl<T> Extend<T> for Counter<T>
where
    T: Eq + Hash,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            *self.map.entry(elem).or_insert(0) += 1;
        }
    }
}

impl<T> FromIterator<T> for Counter<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Counter<T> {
        let mut counter = Counter::new();
        counter.extend(iter);
        counter
    }
}

impl<T> Add for Counter<T>
where
    T: Eq + Hash,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self::new();

        for (key, count) in self.map.into_iter() {
            result[key] += count;
        }

        for (key, count) in other.map.into_iter() {
            result[key] += count;
        }

        result
    }
}

impl<T> Sub for Counter<T>
where
    T: Eq + Hash + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = Self::new();

        for (key, count) in self.map.into_iter() {
            result[key] += count;
        }

        for (key, count) in other.map.into_iter() {
            result[key] -= count;

            if result[key] == 0 {
                result.remove(key);
            }
        }

        result
    }
}

impl<T> Index<T> for Counter<T>
where
    T: Eq + Hash,
{
    type Output = usize;

    fn index(&self, key: T) -> &Self::Output {
        self.map.get(&key).unwrap_or(&0)
    }
}

impl<T> IndexMut<T> for Counter<T>
where
    T: Eq + Hash,
{
    fn index_mut(&mut self, key: T) -> &mut Self::Output {
        self.map.entry(key).or_insert(0)
    }
}

impl<'a, T> IntoIterator for &'a Counter<T>
where
    T: Eq + Hash,
{
    type Item = (&'a T, &'a usize);
    type IntoIter = hash_map::Iter<'a, T, usize>;

    fn into_iter(self) -> hash_map::Iter<'a, T, usize> {
        self.map.iter()
    }
}

pub fn digits<T: Integer + Copy + num_traits::cast::FromPrimitive>(n: T) -> Vec<T> {
    let ten: T = FromPrimitive::from_u32(10).unwrap();
    let mut cur = n;
    let mut result = vec![];

    while cur > Zero::zero() {
        result.push(cur % ten);
        cur = cur / ten;
    }

    result
}

pub fn factorial(n: usize) -> usize {
    (1..=n).product::<usize>()
}

pub fn gcd<T: Integer + Copy>(a: T, b: T) -> T {
    if b > a {
        return gcd(b, a);
    }

    if b == Zero::zero() {
        return a;
    }

    return gcd(b, a % b);
}

pub fn grow_sieve(sieve: &mut Vec<bool>, primes: &mut Vec<usize>, grow_by: usize) {
    let original_size = sieve.len();

    sieve.resize(original_size + grow_by, true);

    sieve[0] = false;
    sieve[1] = false;

    for n in (2..3).chain((3..sieve.len()).step_by(2)) {
        if !sieve[n] {
            continue;
        }

        let mut start = 2 * n;

        if n >= original_size {
            primes.push(n);
        }

        if original_size >= 3 * n {
            start = original_size - (original_size % n);
        }

        for k in (start..sieve.len()).step_by(n) {
            sieve[k] = false;
        }
    }
}

pub fn nth_permutation<T>(mut items: Vec<T>, n: usize) -> Vec<T> {
    if items.len() == 0 {
        return vec![];
    }

    let n_sub_one_fac = factorial(items.len() - 1);
    let index = n / n_sub_one_fac;

    let mut result = vec![items.remove(index)];
    result.extend(nth_permutation(items, n % n_sub_one_fac));

    result
}
