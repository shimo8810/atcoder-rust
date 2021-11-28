use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: Chars,
      K: usize
    }

    if K >= S.len() {
        println!("{}", S.len());
        return;
    }

    let mut ans = 0;
    let mut s = 0;
    let mut t = 0;
    let mut k = 0;
    loop {
        while t < S.len() && k <= K {
            if S[t] == '.' {
                k += 1;
            }
            t += 1;
        }
        if k < K {
            break;
        }
        println!("{}-{}", s, t);
        ans = std::cmp::max(ans, t - s);
        if S[s] == '.' {
            k -= 1;
        }
        s += 1;
    }
    println!("{}", ans);
}
