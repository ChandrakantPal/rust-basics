fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}
