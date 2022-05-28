use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        _W: usize,
        S: [Chars; H]
    }

    let mut koma = vec![];
    for (y, row) in S.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'o' {
                koma.push((x as i32, y as i32));
            }
        }
    }
    let ans = (koma[0].0 - koma[1].0).abs() + (koma[0].1 - koma[1].1).abs();
    println!("{}", ans);
}
