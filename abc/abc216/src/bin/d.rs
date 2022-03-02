use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut a = Vec::new();
    for _ in 0..M {
        input! {
            k: usize,
            b: [u32; k]
        }
        a.push(b);
    }

    let ans = 0;
    println!("{}", ans);
}
