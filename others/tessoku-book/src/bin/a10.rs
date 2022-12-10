use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N],
        D: usize,
    }

    let mut head = vec![0; N + 1];
    let mut tail = vec![0; N + 1];
    for i in 0..N {
        head[i + 1] = A[i].max(head[i]);
        tail[N - i - 1] = A[N - i - 1].max(tail[N - i]);
    }

    for _ in 0..D {
        input! {L: usize, R: usize}
        println!("{}", head[L - 1].max(tail[R]));
    }
}
