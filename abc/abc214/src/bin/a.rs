use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u8
    }

    let ans = match N {
        0..=125 => 4,
        126..=211 => 6,
        _ => 8,
    };
    println!("{}", ans);
}
