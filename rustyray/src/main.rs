// main.rs
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
fn main() {
    let imgx = 800;
    let imgy = 800;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    // Iterate over the coordinates and pixels of the image

    // Scene [camera, lights[] , objs[] ]
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // color = shootray(

        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    imgbuf.save("lol.png");
}
