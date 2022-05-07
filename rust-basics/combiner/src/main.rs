mod args;
use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};
use std::{fs::File, io::BufReader};

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image_1, image_format_1) = find_image_from_path(args.image_1);
    let (image_2, image_format_2) = find_image_from_path(args.image_2);
    // println!("{:?}", args);
    if image_format_1 != image_format_2 {
        return Err();
    }
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}
