use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        X: Usize1,
        Y: Usize1,
        S: [Chars; H]
    }

    let mut ans = 0;
    // println!("{}  {}", 0, X);
    for x in (0..=X).rev() {
        if S[x][Y] == '#' {
            break;
        }
        ans += 1;
    }

    for x in X..H {
        if S[x][Y] == '#' {
            break;
        }
        ans += 1;
    }

    for y in (0..=Y).rev() {
        if S[X][y] == '#' {
            break;
        }
        ans += 1;
    }

    for y in Y..W {
        if S[X][y] == '#' {
            break;
        }
        ans += 1;
    }

    println!("{}", ans - 3);

}
