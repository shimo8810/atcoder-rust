use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _: usize,
        T: Chars
    }

    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;

    for &c in &T {
        if c == 'S' {
            x += dx;
            y += dy;
        } else {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            // (dx, dy) = (dy, -dx);
        }
    }
    println!("{} {}", x, y);
}
