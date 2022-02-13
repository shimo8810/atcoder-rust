use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        H: [u32; N]
    }

    let mut ans = H[0];

    for hs in H.windows(2) {
        if hs[1] > hs[0] {
            ans = hs[1]
        } else {
            break;
        }
    }

    println!("{}", ans);
}
