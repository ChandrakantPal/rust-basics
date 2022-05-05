fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Self {
            image_1: String::new(),
            image_2: String::new(),
            output: String::new(),
        }
    }
}
