use ab_glyph::FontRef;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut, draw_line_segment_mut};
use crate::set::{Set};
use crate::set_util::{Element, Matrix};

static BG_COLOR: Rgb<u8> = Rgb([255,255,255]);
static TEXT_COLOR: Rgb<u8> = Rgb([0,0,0]);
static LINE_COLOR: Rgb<u8> = Rgb([150,150,150]);
static VERTEX_COLOR: Rgb<u8> = Rgb([150,150,150]);

pub fn initialize_blank_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, BG_COLOR);
        }
    }

    img
}

pub fn draw_vertex(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, elem: &mut Element) {
    // Designate font
    let font = FontRef::try_from_slice(
        include_bytes!("/usr/share/fonts/truetype/freefont/FreeMonoBold.ttf")
    )
    .expect("Failed loading font");

    // Find the number of digits of elem
    let mut quotient: u32 = elem.value;

    if quotient / 100 != 0 {
        quotient = 12;
    } else if quotient / 10 != 0 {
        quotient = 17;
    } else {
        quotient = 22;
    }

    // Draw vertex circle
    draw_filled_circle_mut(img, (x, y), 10, VERTEX_COLOR);

    // Draw text inside vertex circle
    draw_text_mut(
        img,
        TEXT_COLOR,
        x + 20,
        y - 12,
        quotient as f32,
        &font,
        &elem.name.trim(),
    );
}

pub fn draw_hasse_diagram(set: &mut Set, matrix: &mut Matrix, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    // Draw vertices and write coordinates to elements.
    let mut matrix_copy = matrix.clone();
    let mut layer = 950;
    loop {
        let min_elts = matrix_copy.extract_minimal_elements();
        let mut spacing = img.width() / (min_elts.len() + 1) as u32;
        let increment = spacing;
        for min_elt in min_elts.iter() {
            // Write coordinates to element
            set.elements[*min_elt].coord = (spacing as f32, layer as f32);
            draw_vertex(img, spacing as i32, layer, &mut set.elements[*min_elt]);
            spacing += increment;
        }

        if matrix_copy.is_empty() {
            break;
        }
        layer -= 100;
    }

    // Draw edges with coordinates in elements.
    matrix_copy = matrix.clone();
    
    loop {
        // TESTING
        matrix_copy.print();
        let min_elts = matrix_copy.extract_minimal_elements();
        for curr in min_elts.iter() {
            for next in 0..matrix.size() {
                if matrix.get(*curr, next) == Some(true) {
                    draw_line_segment_mut(
                        img,
                        set.elements[*curr].coord,
                        set.elements[next].coord,
                        LINE_COLOR
                    );
                }
            }
        }

        if matrix_copy.is_empty() {
            break;
        }
    }
}
