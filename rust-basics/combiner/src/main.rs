mod args;
use args::Args;

fn main() {
    let args = Args::new();
    println!("{:?}", args);
}

fn find_image_from_path(path: String) {}
