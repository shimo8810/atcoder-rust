use proconio::{fastout, input};
use std::collections::{HashSet, VecDeque};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: u32,
        N: u32,
    }

    let mut que = VecDeque::new();

    que.push_back((N, 0));

    let mut set = HashSet::new();

    while let Some((x, i)) = que.pop_front() {


        if x == 1 {
            println!("{}", i);
            return;
        }
        // a で割れるとき
        if x % a == 0 {
            let x = x / a;
            if !set.contains(&x) {
                set.insert(x);
                que.push_back((x, i + 1));
            }
        }
        if x >= 10 {
            let s = x.to_string();
            let c = s.chars().next().unwrap();
            let c2 = s.chars().nth(1).unwrap();
            if c2 != '0' {
                let x: u32 = format!("{}{}", &s[1..], c).parse().unwrap();
                if !set.contains(&x) {
                    set.insert(x);
                    que.push_back((x, i + 1));
                }
            }
        }
    }

    println!("-1");
}
