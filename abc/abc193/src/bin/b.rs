use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut ans = std::u32::MAX;

    for _ in 0..N {
        input! {A: u32, P: u32, X: u32}
        // 在庫計算
        if X <= A {
            continue;
        }
        ans = ans.min(P);
    }

    if ans == std::u32::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
