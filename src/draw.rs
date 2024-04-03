use ab_glyph::FontRef;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_hollow_circle_mut;
use imageproc::drawing::draw_text_mut;

pub fn initialize_blank_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = RgbImage::new(width, height);
    let color = Rgb([255,255,255]);
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, color);
        }
    }

    img
}

pub fn draw_vertex(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, elem: &str) {
    // Designate font
    let font = FontRef::try_from_slice(
        include_bytes!("/usr/share/fonts/truetype/freefont/FreeMonoBold.ttf")
    )
    .expect("Failed loading font");

    // Find the number of digits of elem
    let mut quotient: u32 = elem.trim().parse().expect("Failed to trim and parse.");

    let mut offset: i32 = 4;

    if quotient / 100 != 0 {
        quotient = 20;
        offset = 1;
    } else if quotient / 10 != 0 {
        quotient = 25;
        offset = 2;
    } else {
        quotient = 30;
    }

    // Draw vertex circle
    draw_hollow_circle_mut(img, (x, y), 30, Rgb([0,0,0]));

    // Draw text inside vertex circle
    draw_text_mut(
        img,
        Rgb([0,0,0]),
        x - (24 / offset),
        y - (24 / 2),
        quotient as f32,
        &font,
        &elem.trim(),
    );
}

//pub fn make_chain(elem: relation: Vec<(u32,u32)>) {
        
//}

