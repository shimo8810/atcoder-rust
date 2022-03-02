use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      M: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..M {
        input! {a: Usize1, b: Usize1}
        map.entry(a).or_insert_with(Vec::new).push(b);
        map.entry(b).or_insert_with(Vec::new).push(a);
    }

    let mut ans = 0;
    for i in 0..N {
        if let Some(v) = map.get(&i) {
            if v.iter().filter(|&&x| x < i).count() == 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
