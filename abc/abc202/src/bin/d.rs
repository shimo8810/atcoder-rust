use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
        mut K: usize
    }
    let mut ans = vec![];

    let mut perm = vec![vec![0; B + 1]; A + 1];
    perm[0][0] = 1;

    for i in 0..=A {
        perm[i][0] = 1;
    }
    for i in 0..=B {
        perm[0][i] = 1;
    }
    for a in 1..=A {
        for b in 1..=B {
            perm[a][b] = perm[a][b - 1] + perm[a - 1][b];
        }
    }

    for _ in 0..(A + B) {
        let x = if A == 0 {
            'b'
        } else if B == 0 {
            'a'
        } else if K <= perm[A - 1][B] {
            A -= 1;
            'a'
        } else {
            K -= perm[A - 1][B];
            B -= 1;
            'b'
        };
        ans.push(x);
    }
    let ans: String = ans.into_iter().collect();
    println!("{}", ans);
}
