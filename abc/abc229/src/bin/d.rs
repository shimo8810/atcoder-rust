use proconio::{fastout, input, marker::Chars};
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: Chars,
      K: usize
    }

    let mut v = vec![];
    let mut n = 0;
    for c in S.iter() {
        v.push(n);
        if *c == '.' {
            n += 1;
        }
    }
    v.push(n);

    let mut r = 0;
    let mut ans = 0;

    for l in 0..S.len() {
        while r < S.len() && v[r + 1] - v[l] <= K {
            r += 1;
        }

        ans = std::cmp::max(ans, r - l);
    }
    println!("{}", ans);
}
