use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [i32; N],
        B: [i32; N]
    }

    let ans = A.iter().zip(B.iter()).fold(0, |acc, (a, b)| acc + a * b);
    if ans == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
