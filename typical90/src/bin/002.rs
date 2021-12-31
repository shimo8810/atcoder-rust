use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u32
    }

    // 全探索
    if N % 2 != 0 {
        return;
    }
    // 0 -> )
    // 1 -> (
    let dic = [")", "("];
    let mut ans = Vec::new();
    for bits in 0..(2usize.pow(N)) {
        let bits: Vec<_> = (0..N).map(|i| 0x1 & (bits >> i)).collect();
        if bits.len() / 2 != bits.iter().filter(|&&x| x == 1).count() {
            continue;
        }
        if !bits
            .iter()
            .scan(0i32, |d, &x| {
                *d += if x == 1 { 1i32 } else { -1i32 };
                Some(*d)
            })
            .all(|x| x >= 0)
        {
            continue;
        }

        ans.push(bits.iter().map(|&x| dic[x]).join(""));
    }

    ans.sort();
    for a in ans.iter() {
        println!("{}",a);
    }
}
