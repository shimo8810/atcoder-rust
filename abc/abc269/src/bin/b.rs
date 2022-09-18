use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: [Chars;10],
    }

    let mut f1 = false;

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    for (i, row) in S.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if v == '#' {
                if !f1 {
                    a = i;
                    c = j;
                    f1 = true;
                }
                b = i;
                d = j;
            }
        }
    }

    println!("{} {} {} {}", a + 1, b + 1, c + 1, d + 1);
}
