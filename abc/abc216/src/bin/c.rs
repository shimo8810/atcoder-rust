use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: u64
    }

    let mut ans = Vec::new();
    while N > 0 {
        ans.push(N & 0x1);
        N >>= 1;
    }

    let ans = ans
        .iter()
        .rev()
        .map(|&x| if x == 1 { "A" } else { "" })
        .collect::<Vec<_>>()
        .join("B");

    println!("{}", ans);
}
