use proconio::input;

fn main() {
    input! { n: usize, k: usize }

    println!(
        "{}",
        (1..=n)
            .flat_map(|x| {
                (1..=n).filter(move |y| {
                    k >= x + y && {
                        let z = k - x - y;

                        1 <= z && z <= n
                    }
                })
            })
            .count()
    );
}
