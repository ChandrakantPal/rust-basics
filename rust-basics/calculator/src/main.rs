use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    println!("{:?} {} {}", first, operator, second);
}
