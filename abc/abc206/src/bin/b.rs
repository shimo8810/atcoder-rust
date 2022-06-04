use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u64,
    }

    let mut ng = 0;
    let mut ok = N;
    while ok - ng > 1 {
        let md = (ok + ng) / 2;

        if (1 + md) * md / 2 >= N {
            ok = md;
        } else {
            ng = md;
        }
    }
    println!("{}", ok);
}
