use proconio::{fastout, input, marker::Usize1};

const INF: u64 = std::u64::MAX;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize
    }

    let mut dist = vec![vec![INF; N]; N];

    for _ in 0..M {
        input! {A: Usize1, B: Usize1, C: u64}
        dist[A][B] = C;
    }
    for i in 0..N {
        dist[i][i] = 0;
    }

    let mut ans = 0;
    for k in 0..N {
        let mut d = vec![vec![0; N]; N];
        for i in 0..N {
            for j in 0..N {
                d[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
                if d[i][j] != INF {
                    ans += d[i][j];
                }
            }
        }
        dist = d;
    }

    println!("{}", ans);
}
