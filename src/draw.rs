use ab_glyph::FontRef;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_hollow_circle_mut, draw_text_mut, draw_line_segment_mut};
use std::collections::HashMap;
use crate::set::{Set};
use crate::set_util::{Element, Matrix};

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

pub fn draw_vertex(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, elem: &mut Element) {
    // Designate font
    let font = FontRef::try_from_slice(
        include_bytes!("/usr/share/fonts/truetype/freefont/FreeMonoBold.ttf")
    )
    .expect("Failed loading font");

    // Find the number of digits of elem
    let mut quotient: u32 = elem.value;

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

    // Update Element coordinates
    elem.coord = (x as u32, y as u32); // Need to find out how to stop doing so much type casting

    // Draw text inside vertex circle
    draw_text_mut(
        img,
        Rgb([0,0,0]),
        x - (24 / offset),
        y - (24 / 2),
        quotient as f32,
        &font,
        &elem.name.trim(),
    );
}

pub fn draw_hasse_diagram(set: &mut Set, matrix: &Matrix, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut hasse_map = HashMap::new();
    let mut prev_min_elts = Vec::new();
    let mut matrix_copy = matrix.clone();

    let mut i = 900;
    loop {
        // TESTING
        matrix_copy.print();

        // Build hasse map
        let min_elts = matrix_copy.find_minimal_elements();
        let mut spacing = (img.width() / (min_elts.len() + 1) as u32) as i32;
        let increment = spacing;
        for curr in min_elts.iter() {
            hasse_map.insert(*curr, Vec::new());
            for prev in prev_min_elts.iter() {
                if matrix.get(*prev, *curr) == Some(true) {
                    hasse_map.get_mut(curr).expect("Key not in HashMap").push(*prev);
                }
            }

            draw_vertex(img, spacing, i, &mut set.elements[*curr]);
            for prev in hasse_map.get(curr).expect("KeyValueError") {
                let curr_coord = (set.elements[*curr].coord.0 as f32, set.elements[*curr].coord.1 as f32);
                let prev_coord = (set.elements[*prev].coord.0 as f32, set.elements[*prev].coord.1 as f32); 
                draw_line_segment_mut(img, curr_coord, prev_coord, Rgb([0,0,0]));
            }
            spacing += increment;
        }

        matrix_copy.remove_minimal_elements(&min_elts);
        if matrix_copy.is_empty() {
            break;
        }

        prev_min_elts = min_elts.clone();
        i -= 100;
    }
}
