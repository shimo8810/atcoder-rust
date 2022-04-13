use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (x1, y1): (i8, i8),
        (x2, y2): (i8, i8),
        (x3, y3): (i8, i8),
    }
    let x4 = if x1 == x2 {
        x3
    } else if x2 == x3 {
        x1
    } else {
        x2
    };

    let y4 = if y1 == y2 {
        y3
    } else if y2 == y3 {
        y1
    } else {
        y2
    };

    println!("{} {}", x4, y4);
}
