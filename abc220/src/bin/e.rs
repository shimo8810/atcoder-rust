use proconio::input;

const M: usize = 998244353;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize
    }
    // power of two
    let mut pt = vec![1; 2 * N + 1];
    for i in 1..=(2 * N) {
        pt[i] = (pt[i - 1] * 2) % M;
    }
    let mut ans = 0;

    for k in 0..N {
        //
        if k + D < N {
            ans = (ans + 2 * pt[D]) % M;
        }
        println!("{} + {} < {}, {}", k, D, N, N - k);
        let n = if k + D + 1 < N {
            N - k
        } else {
            2 * N - 2 * k - D
        };
        ans = (ans + n * pt[D - 1]) % M;
    }

    println!("{}", ans);
}
