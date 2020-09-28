use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u64,
    }

    let ans = match N % 10 {
        //
        x if x == 3 => "bon",
        x if [0, 1, 6, 8].contains(&x) => "pon",
        _ => "hon",
    };

    println!("{}", ans);
}
