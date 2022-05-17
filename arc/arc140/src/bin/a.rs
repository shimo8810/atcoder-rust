use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: Chars
    }
    let mut divs = vec![];
    let mut n = 1;
    while n * n <= N {
        if N % n == 0 {
            divs.push(n);
            if N / n != n {
                divs.push(N / n);
            }
        }
        n += 1;
    }
    divs.sort_unstable();

    for &n in &divs {
        let m = N / n;
        let mut k = 0;
        for i in 0..n {
            let mut map = HashMap::new();
            for j in 0..m {
                *map.entry(S[j * n + i]).or_insert(0) += 1;
            }
            let (_, x) = map.into_iter().max_by_key(|&(_, x)| x).unwrap();
            k += m - x;
        }
        if k <= K {
            println!("{}", n);
            return;
        }
    }
}
