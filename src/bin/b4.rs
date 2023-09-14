use proconio::input;

fn main() {
    input! { s: String }

    let mut x = 0;

    for c in s.chars() {
        x *= 2;
        x += if c == '0' { 0 } else { 1 };
    }

    println!("{}", x);
}
