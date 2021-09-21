use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: [Chars; N],
        S: [Chars; N]
    }
    for x in 0..N {
        for y in 0..N {
            println!("{}, {}", T[x][y], S[y][x]);
        }
    }
    let ans = 0;
    println!("{}", ans);
}
