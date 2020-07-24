fn main() {
    const MAX_N: usize = 1_000_000;

    let mut d = vec![1; MAX_N + 1];

    for n in 2..=MAX_N {
        for i in (2*n..=MAX_N).step_by(n) {
            d[i] += n;
        }
    }

    let mut chain_length = vec![None; MAX_N + 1];
    chain_length[1] = Some(0);

    'next_n: for n in 2..=MAX_N {
        if chain_length[n].is_some() {
            continue;
        }

        if d[n] == 1 {
            continue;
        }

        let mut cur = n;
        let mut seen = vec![n];

        while chain_length[cur].is_none() {
            cur = d[cur];

            if seen.contains(&cur) {
                let cur_pos = seen.iter().position(|&x| x == cur).unwrap();

                for i in cur_pos..seen.len() {
                    chain_length[seen[i]] = Some(seen.len() - cur_pos);
                }

                continue 'next_n;
            } else if cur > MAX_N || chain_length[cur].is_some() {
                chain_length[n] = Some(0);
                continue 'next_n;
            }

            seen.push(cur);
        }
    }

    println!("{:?}", chain_length.iter()
                                 .enumerate()
                                 .max_by(|(i, l1), (j, l2)| l1.cmp(l2)
                                                              .then(j.cmp(i)))
                                 .map(|(n, _)| n)
                                 .unwrap());
}
