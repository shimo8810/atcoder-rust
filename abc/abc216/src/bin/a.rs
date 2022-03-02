use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: String
    }

    let mut v = X.split(".");
    let x = v.next().unwrap();
    let y = match v.last().unwrap().parse::<u8>().unwrap() {
        y if y < 3 => "-",
        y if y < 7 => "",
        _ => "+",
    };
    println!("{}{}", x, y);
}
