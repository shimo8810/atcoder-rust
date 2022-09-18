use std::io::{stdin, stdout, BufReader, Write};

use proconio::{input, source::line::LineSource};

#[allow(non_snake_case)]
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        N: usize,
    }

    let mut head = 0;
    let mut tail = N;
    let mut mid = (head + tail) / 2;

    while tail - head > 1 {
        println!("? {} {} {} {}", head + 1, mid, 1, N);

        input! {
            from &mut source,
            x: usize,
        }

        if mid - head > x {
            tail = mid;
        } else {
            head = mid;
        }
        mid = (head + tail) / 2;
    }
    let ansx = tail;

    let mut head = 0;
    let mut tail = N;
    let mut mid = (head + tail) / 2;

    while tail - head > 1 {
        println!("? {} {} {} {}", 1, N, head + 1, mid);

        input! {
            from &mut source,
            x: usize,
        }

        if mid - head > x {
            tail = mid;
        } else {
            head = mid;
        }
        mid = (head + tail) / 2;
    }
    let ansy = tail;

    println!("! {} {}", ansx, ansy);
    stdout().flush().unwrap();
}
