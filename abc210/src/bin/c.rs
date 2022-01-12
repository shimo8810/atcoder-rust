use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        C: [u32; N]
    }

    let mut map = HashMap::new();

    for &c in C.iter().take(K) {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..=(N - K) {
        ans = ans.max(map.len());
        if i + K < N {
            *map.entry(C[i + K]).or_insert(0) += 1;
        }
        *map.get_mut(&C[i]).unwrap() -= 1;
        if *map.get(&C[i]).unwrap() == 0 {
            map.remove(&C[i]);
        }
    }

    println!("{}", ans);
}
