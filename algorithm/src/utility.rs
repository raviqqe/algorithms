use std::fmt::Debug;

pub fn print_table<T: Debug>(xs: &[Vec<T>]) {
    for xs in xs {
        for x in xs {
            print!("{:?}\t", x);
        }

        println!();
    }
}
