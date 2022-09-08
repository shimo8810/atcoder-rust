use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (H, W): (usize, usize),
        G: [Chars; H]
    }
    let mut set = HashSet::new();
    let mut x = 0i32;
    let mut y = 0i32;
    loop {
        if set.contains(&(x, y)) {
            println!("-1");
            break;
        } else {
            set.insert((x, y));
        }

        let (dy, dx) = match G[y as usize][x as usize] {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            _ => (0, 1),
        };
        if x + dx == -1 || x + dx == W as i32 || y + dy == -1 || y + dy == H as i32 {
            println!("{} {}", y + 1, x + 1);
            break;
        }
        x += dx;
        y += dy;
    }
}
