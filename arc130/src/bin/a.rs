use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      _N: usize,
      S: Chars
    }

    let mut v = vec![];
    let mut t = S[0];
    let mut n = 0u64;

    for &c in S.iter() {
        if t == c {
            n += 1;
        } else {
            v.push(n);
            t = c;
            n = 1;
        }
    }

    v.push(n);
    let ans:u64 = v.iter().map(|n| n * (n - 1) / 2).sum();
    println!("{}", ans);
}
