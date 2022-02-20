use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      K: i64,
      A: [i64; N]
    }

    let mut map = HashMap::new();
    map.entry(0).or_insert_with(Vec::new).push(0);
    let mut S = vec![0; N + 1];
    let mut ans = 0;

    for (i, a) in A.iter().enumerate() {
        S[i + 1] = S[i] + a;
        map.entry(S[i + 1]).or_insert_with(Vec::new).push(i + 1);
    }

    for (i, s) in S.iter().enumerate() {
        if let Some(v) = map.get(&(s - K)) {
            ans += match v.binary_search_by(|probe| probe.cmp(&i)) {
                Ok(x) => x,
                Err(x) => x,
            };
        }
    }
    println!("{}", ans);
}
