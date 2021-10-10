use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: u32,
        A: [u32; N]
    }

    let ans = A.iter().filter(|&x| x < &P).count();
    println!("{}", ans);
}
