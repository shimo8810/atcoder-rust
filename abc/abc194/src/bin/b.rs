use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        AB: [(u32, u32); N]
    }

    let mut ans = std::u32::MAX;
    // 仕事Aを割り振る人
    for i in 0..N {
        // 仕事Bを割り振る人
        for j in 0..N {
            let cost = if i == j {
                AB[i].0 + AB[j].1
            } else {
                AB[i].0.max(AB[j].1)
            };
            ans = ans.min(cost);
        }
    }
    println!("{}", ans);
}
