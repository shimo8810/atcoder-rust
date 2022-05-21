use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }
    let S: Vec<_> = S
        .into_iter()
        .map(|t| t.into_iter().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect();

    let mut ans = std::usize::MAX;
    for x in 0..10 {
        let mut set = HashSet::new();
        for s in &S {
            let mut i = s.iter().position(|&y| y == x).unwrap();
            while set.contains(&i) {
                i += 10;
            }
            set.insert(i);
        }
        ans = ans.min(set.into_iter().max().unwrap());
    }

    println!("{}", ans);
}
