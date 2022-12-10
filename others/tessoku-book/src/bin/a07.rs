use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        D: usize,
        N: usize,
    }

    let mut cs = vec![0i32; D + 1];
    for _ in 0..N {
        input! {L: Usize1, R: Usize1}
        cs[L] += 1;
        cs[R + 1] -= 1;
    }

    let mut prev = 0;
    for d in cs.iter().take(D) {
        println!("{}", d + prev);
        prev += d;
    }
}
