use std::collections::HashMap;

use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize
    }

    let mut ans: Vec<_> = (1..=N).collect();
    let mut map: HashMap<_, _> = (1..=N).map(|x| (x, x - 1)).collect();
    for _ in 0..Q {
        input! {mut x: usize}
        let &i = map.get(&x).unwrap();
        let j = if i + 1 == N { i - 1 } else { i + 1 };
        let y = ans[j];
        *map.get_mut(&x).unwrap() = j;
        *map.get_mut(&y).unwrap() = i;
        ans.swap(i, j);
    }
    for x in &ans {
        print!("{} ", x);
    }
}
