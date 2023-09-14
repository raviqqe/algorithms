use proconio::input;

fn main() {
    input! { mut n: usize }

    let mut xs = vec![];

    if n == 0 {
        xs.push(0);
    } else {
        while n != 0 {
            xs.push(n % 2);
            n = n / 2;
        }
    }

    while xs.len() < 10 {
        xs.push(0)
    }

    for x in xs.into_iter().rev() {
        print!("{}", x);
    }
}
