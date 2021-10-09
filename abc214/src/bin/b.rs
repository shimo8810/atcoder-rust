use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: u32,
        T: u32
    }

    let mut ans = 0;
    for a in 0..=S {
        for b in 0..=(S - a) {
            for c in 0..=(S - a - b) {
                if a * b * c <= T {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
