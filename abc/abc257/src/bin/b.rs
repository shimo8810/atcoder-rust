use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        Q: usize,
        mut A: [Usize1; K],
        L: [Usize1; Q]
    }

    for &l in &L {
        let i = A[l];

        // 右端なら何もしない
        if i == N - 1 {
            continue;
        }

        // 隣にいれば何もしない
        if l != K - 1 {
            let j = A[l + 1];
            if i + 1 == j {
                continue;
            }
        }

        A[l] = i + 1;
    }

    for a in &A {
        print!("{} ", a + 1);
    }
}
