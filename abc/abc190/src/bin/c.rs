use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
        K: usize,
        CD: [(Usize1, Usize1); K]
    }

    let mut ans = 0;
    for bit in 0..(1 << K) {
        let mut sara = vec![0; N];
        for (i, j) in (0..K).map(|x| (bit >> x) & 1).rev().enumerate() {
            let idx = if j == 0 { CD[i].0 } else { CD[i].1 };
            sara[idx] += 1;
        }
        ans = AB
            .iter()
            .filter(|(a, b)| sara[*a] > 0 && sara[*b] > 0)
            .count()
            .max(ans);
    }

    println!("{}", ans);
}
