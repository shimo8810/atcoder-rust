use proconio::{input, source::line::LineSource};
use std::collections::HashSet;
use std::io::{stdout, Write};

#[allow(non_snake_case)]
fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(stdin.lock());

    input! { from &mut source, N: u32 }

    let mut set: HashSet<_> = (1..=(2 * N + 1)).collect();

    loop {
        let &x = set.iter().next().unwrap();
        set.remove(&x);
        println!("{}", x);
        stdout().flush().unwrap();

        input! { from &mut source, x: u32}
        if x == 0 {
            break;
        }
        set.remove(&x);
    }
}
