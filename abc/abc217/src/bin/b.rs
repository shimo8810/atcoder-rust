use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: [String; 3]
    }
    for t in ["ABC", "ARC", "AGC", "AHC"].iter() {
        if S.iter().find(|&x| x == t).is_none() {
            println!("{}", t);
            break;
        }
    }
}
