use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        T: usize,
        N: usize,
    }
    let mut Z = vec![0; T + 1];
    for _ in 0..N {
        input! {L: usize, R: usize}
        Z[L] += 1;
        Z[R] -= 1;
    }

    let mut n = 0;
    for &z in Z.iter().take(T) {
        n += z;
        println!("{}", n);
    }
}
