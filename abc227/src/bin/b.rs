use proconio::{fastout, input};

#[allow(non_snake_case)]
// #[fastout]
fn main() {
    input! {
      N: i64,
    }
    // let mut list = vec![vec![]; N];
    for _ in 0..(N - 1) {
        input! {a: i64, b:i64}
        println!("N{} {} {}", N, a, b);
        // list[a].push(b);
        // list[b].push(a);
    }
    // let mut dist = vec![-1; N];
    // let mut stack = vec![0];

    // while let Some(u) = stack.pop() {
    //     for &v in list[u].iter() {
    //         if dist[v] < 0 {
    //             dist[v] = dist[u] + 1;
    //             stack.push(v);
    //         }
    //     }
    // }

    // println!("{:?}", dist);
    // let ans = 0;
    // println!("{}", ans);
}
