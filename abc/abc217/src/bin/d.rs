use proconio::input;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        Q: usize,
        cx: [(u8, usize); Q]
    }
    let mut tree = BTreeSet::new();
    tree.insert(0);
    tree.insert(L);

    for (c, x) in cx {
        if c == 2 {
            // output proc
            let left = tree.range(..x).next_back().unwrap();
            let right = tree.range(x..).next().unwrap();

            println!("{}", right - left);
        } else {
            // cutting proc
            tree.insert(x);
        }
    }
}
