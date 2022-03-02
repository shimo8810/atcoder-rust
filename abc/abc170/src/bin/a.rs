use proconio::input;

fn main() {
    input! {
        x: [u64; 5],
    }

    for (i, &e) in x.iter().enumerate() {
        if e == 0 {
            println!("{}", i + 1);
        }
    }
}
