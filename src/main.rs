use bmp::{open, px, Image, Pixel};
use rand::Rng;

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100); // create a bmp image width, height
    image.set_pixel(50, 50, Pixel::new(255, 255, 255)); // set a pixel at x,y, with r,g,b (0-255 u8)
    image.save(path).unwrap(); // saves image at path (string - full or relative path)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("use with args ");
    }
    match args[1].as_str() {
        "pixel" => draw_pixel("pixel.bmp"),
        "diagonal" => draw_diagonal("diagonal.bmp"),
        "house" => draw_house("house_.bmp"),
        "wtf" => draw_wtf("wtf.bmp", 0, 1, 2),
        "wtfall" => draw_wtfall("wtf/wtf"),
        "random" => draw_random("random.bmp"),
        _ => {
            panic!("unknonwn comand")
        }
    }
}

fn draw_diagonal(path: &str) {
    const SIZE: u32 = 100;
    let mut image = Image::new(SIZE, SIZE);
    for i in 0..(SIZE) {
        image.set_pixel(i, i, px!(255, 255, 255));
    }
    image.save(path).unwrap();
}

fn draw_house(path: &str) {
    let mut image = Image::new(100, 100);
}

fn draw_filled_rect(image: &mut Image, x1: u32, y1: u32, x2: u32, y2: u32, pixel: Pixel) {
    for x in (x1)..(x2) {
        for y in y1..y2 {
            image.set_pixel(x, y, pixel);
        }
    }
}

fn draw_triangle(image: &mut Image, x1: u32, y1: u32, x2: u32, pixel: Pixel) {
    for x in x1..x2 {
        for y in y1..(x - x1) {
            image.set_pixel(x, y, pixel);
        }
    }
}

fn draw_wtf(path: &str, r: usize, g: usize, b: usize) {
    let mut image = Image::new(500, 500);
    let mut buf: Vec<u8> =
        Vec::with_capacity((image.get_width() * image.get_height()) as usize + 3);
    let mut rand = rand::rng();
    for _ in 0..buf.capacity() {
        buf.push(rand.random());
    }
    for (i, idx) in buf[..].windows(3).enumerate() {
        image.set_pixel(
            i as u32 % image.get_width(),
            i as u32 / image.get_width(),
            px!(idx[r], idx[g], idx[b]),
        );
    }
    image.save(path).unwrap();
}

fn draw_wtfall(path: &str) {
    for r in 0..3 {
        for g in 0..3 {
            for b in 0..3 {
                draw_wtf(&format!("{path}{r}{g}{b}.bmp"), r, g, b);
            }
        }
    }
}

fn draw_random(path: &str) {
    let mut image = Image::new(500, 500);
    let mut rand = rand::rng();
    for x in 0..image.get_width() {
        for y in 0..image.get_height() {
            image.set_pixel(
                x,
                y,
                Pixel::new(rand.random(), rand.random(), rand.random()),
            );
        }
    }
    image.save(path).unwrap();
}
