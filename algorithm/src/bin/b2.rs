use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    println!(
        "{}",
        if (n..=m).any(|i| 100 % i == 0) {
            "Yes"
        } else {
            "No"
        }
    );
}
