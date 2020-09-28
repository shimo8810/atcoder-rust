use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut arr = [0; 4];
    let case = ["AC", "WA", "TLE", "RE"];

    for s in ss.iter() {
        match &s[..] {
            "AC" => arr[0] += 1,
            "WA" => arr[1] += 1,
            "TLE" => arr[2] += 1,
            "RE" => arr[3] += 1,
            _ => {}
        }
    }

    for i in 0..4 {
        println!("{} x {}", case[i], arr[i]);
    }
}
