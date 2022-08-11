use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [Usize1; N]
    }
    let mut eq = vec![0; N];
    for (i, &a) in A.iter().enumerate().rev() {
        if a == i {
            if i == N - 1 {
                eq[i] = 1;
            } else {
                eq[i] = eq[i + 1] + 1;
            }
        } else if i != N - 1 {
            eq[i] = eq[i + 1];
        }
    }

    let mut ans = 0u64;

    for (i, &a) in A.iter().enumerate().take(N - 1) {
        if i == a {
            ans += eq[i + 1];
        } else if A[a] == i && i < a {
            ans += 1;
        }
    }
    println!("{}", ans);
}
