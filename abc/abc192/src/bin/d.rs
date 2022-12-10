use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: Chars,
        M: u128
    }

    if X.len() == 1 {
        let ans = if (X[0] as u128 - 48) <= M { 1 } else { 0 };
        println!("{}", ans);
        return;
    }

    let X: Vec<_> = X.into_iter().map(|c| c as u128 - 48).collect();
    let &mx = X.iter().max().unwrap();
    let mut ok = mx;
    let mut ng = M + 1;

    while ng - ok > 1 {
        let md = (ng + ok) / 2;

        let mut xx = 0;

        for &x in X.iter() {
            if xx > M / md {
                xx = M + 1;
            } else {
                xx = xx * md + x;
            }
        }
        if xx <= M {
            ok = md;
        } else {
            ng = md;
        }
    }

    println!("{}", ok - mx);
}
