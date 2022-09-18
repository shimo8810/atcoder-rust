use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: u64,
    }
    let mut list = vec![];
    for i in 0..60 {
        if X & (1 << i) != 0 {
            list.push(X & (1 << i));
        }
    }

    let n = list.len();
    for bit in 0..(1 << n) {
        let mut v = vec![];
        for i in 0..n {
            if bit & (1 << i) != 0 {
                v.push(i);
            }
        }
        let mut ans = 0;
        for &i in v.iter() {
            ans += list[i as usize];
        }
        println!("{}", ans);
    }
}
