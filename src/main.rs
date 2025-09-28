use bmp::{Image, Pixel};

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100); // create a bmp image width, height
    image.set_pixel(50, 50, Pixel::new(255, 255, 255)); // set a pixel at x,y, with r,g,b (0-255 u8)
    image.save(path).unwrap(); // saves image at path (string - full or relative path)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("use wiht args ");
    }
    match args[1].as_str() {
        "pixel" => draw_pixel("pixel.bmp"),
        _ => {  panic!("unknonwn comand")}
    }
}
