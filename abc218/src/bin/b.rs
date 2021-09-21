use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        P: [u8; 26]
    }

    let ans: String = P.iter().map(|&x| (x - 1 + 'a' as u8) as char).collect();
    println!("{}", ans);
}
