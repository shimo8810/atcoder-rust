use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        R: Usize1,
        C: Usize1
    }

    let mut ans = 0;

    if R > 0 {
        ans += 1;
    }
    if C > 0 {
        ans += 1;
    }
    if R + 1 < H {
        ans += 1;
    }
    if C + 1 < W {
        ans += 1;
    }

    println!("{}", ans);
}
