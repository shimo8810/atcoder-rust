use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, u64); M]
    }

    let mut dist = vec![vec![std::u64::MAX; N]; N];
    for &(a, b, c) in &ABC {
        dist[a][b] = c;
        dist[b][a] = c;
    }

    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
            }
        }
    }

    let mut ans = 0;

    for &(a, b, c) in &ABC {
        if (0..N).into_iter().any(|x| dist[a][x] + dist[x][b] <= c) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
