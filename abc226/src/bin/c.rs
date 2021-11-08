use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
    }

    let mut map = HashMap::new();
    for i in 0..N {
        input! {
            T: u64,
            K: usize,
            A: [Usize1; K]
        }
        map.entry(i).or_insert((i, T, A));
    }
    let mut que = VecDeque::new();
    let mut set = HashSet::new();
    que.push_back(map.get(&(N - 1)).unwrap());

    while let Some((i, _, x)) = que.pop_front() {
        set.insert(i);
        for n in x.iter() {
            if !set.contains(n) {
                que.push_back(map.get(n).unwrap());
            }
        }
    }

    let ans = set.iter().fold(0, |acc, x| acc + map.get(&x).unwrap().1);
    println!("{:?}", ans);
}
