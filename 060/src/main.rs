use std::collections::{HashMap, HashSet};

use euler_utils::grow_sieve;

fn is_prime(sieve: &Vec<bool>, primes: &Vec<usize>, n: usize) -> bool {
    if n < sieve.len() {
        return sieve[n];
    }

    let last_prime = primes[primes.len() - 1];

    if last_prime * last_prime < n {
        panic!("welp it's just too big {}", n);
    }

    for p in primes {
        if n % p == 0 {
            return false;
        }
     }

    return true;
}

fn main() {
    let mut sieve = vec![];
    let mut primes = vec![];

    let mut edgelist = HashMap::new();

    grow_sieve(&mut sieve, &mut primes, 10_000);

    let initial_prime_len = primes.len();

    for i in 0..initial_prime_len {
        for j in i+1..initial_prime_len {
            let ij = format!("{}{}", primes[i], primes[j]).parse::<usize>().unwrap();
            let ji = format!("{}{}", primes[j], primes[i]).parse::<usize>().unwrap();

            let last_prime = primes[primes.len() - 1];

            if last_prime * last_prime < ij || last_prime * last_prime < ji {
                grow_sieve(&mut sieve, &mut primes, 10_000);
            }

            if is_prime(&sieve, &primes, ij) && is_prime(&sieve, &primes, ji) {
                edgelist.entry(primes[i]).or_insert(HashSet::new()).insert(primes[j]);
                edgelist.entry(primes[j]).or_insert(HashSet::new()).insert(primes[i]);
            }
        }
    }

    let mut cliques = HashSet::new();

    for (a, edges) in &edgelist {
        for b in edges {
            if b > a {
                cliques.insert(vec![*a, *b]);
            }
        }
    }

    for _ in 3..=5 {
        let mut new_cliques = HashSet::new();

        for clique in &cliques {
            for prime in clique {
                for potential_prime in &edgelist[prime] {
                    if clique.contains(potential_prime) {
                        continue;
                    }

                    let set = clique.iter().cloned().collect::<HashSet<_>>();

                    if edgelist[potential_prime].intersection(&set).collect::<Vec<_>>().len() == clique.len() {
                        let mut new_clique = clique.clone();
                        new_clique.push(*potential_prime);
                        new_clique.sort();
                        new_cliques.insert(new_clique);
                    }
                }
            }
        }

        cliques = new_cliques;
    }

    let mut cliques = cliques.into_iter().collect::<Vec<_>>();
    cliques.sort_by(|a, b| a.iter().sum::<usize>().cmp(&b.iter().sum::<usize>()));

    println!("{}", cliques[0].iter().sum::<usize>());
}
