use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: usize,
        A: [usize; N],
    }

    let mut dist = vec![std::usize::MAX; N + 1]; // 街1 からの距離
    dist[1] = 0;
    let mut step = 0;
    let mut l1 = 0; // 1 -> 閉路開始点までの距離
    let mut l2 = 0; // 閉路長

    let mut node = 1;
    for _ in 0..N {
        step += 1;
        node = A[node - 1];
        if dist[node] < std::usize::MAX {
            l1 = dist[node];
            l2 = step - dist[node];
            break;
        }
        dist[node] = step;
    }

    let ans = if K < l1 {
        dist.iter().position(|&x| x == K).unwrap()
    } else {
        let a = (K - l1) % l2 + l1;
        dist.iter().position(|&x| x == a).unwrap()
    };
    println!("{}", ans);
}
