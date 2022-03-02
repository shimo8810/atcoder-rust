use proconio::input;
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: i64,
        mut K: i64,
        D: i64,
    }

    if X < 0 {
        X *= -1;
    }

    let mut ans = 0;

    if K <= X / D {
        ans = X - D * K;
    } else {
        K -= X / D;
        X -= (X / D) * D;
        if K % 2 == 0 {
            ans = X;
        } else {
            ans = -X + D;
        }
    }

    println!("{}", ans);
}
