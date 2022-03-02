use proconio::{fastout, input, marker::Usize1};
use std::collections::{HashMap, HashSet};

const K: u64 = 998244353;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      M: usize
    }

    let mut map = HashMap::new();
    for _ in 0..M {
        input! {u: Usize1, v: Usize1}
        map.entry(u).or_insert_with(Vec::new).push(v);
        map.entry(v).or_insert_with(Vec::new).push(u);
    }

    if N != M {
        println!("0");
        return;
    }

    let mut checked = HashSet::new();
    let mut ans = 1;
    // while let Some(z) =
    for n in 0..N {
        if !checked.contains(&n) {
            checked.insert(n);
            let mut stack = vec![n];
            let mut e = 0;
            let mut k = 1;

            while let Some(u) = stack.pop() {
                if let Some(vs) = map.get(&u) {
                    e += vs.len();
                    for &v in vs.iter() {
                        if !checked.contains(&v) {
                            k += 1;
                            checked.insert(v);
                            stack.push(v);
                        }
                    }
                }
            }
            ans = if k * 2 == e { (ans * 2) % K } else { 0 };
        }
    }
    println!("{}", ans);
}
