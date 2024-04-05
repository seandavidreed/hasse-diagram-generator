//use std::collections::VecDeque;

mod draw;
use crate::draw::*;

mod set;
use crate::set::*;

fn main() {

    // Initialize set with user input.
    let mut set: Vec<Element> = get_set();
    set.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Initialize relation with user input.
    let relation = Relation::build_from_set(&set);

    // Find minimal elements in relation.
    let minimal_elements: Vec<Element> = find_minimal_elements(&set, &relation);

    // Initialize image.
    let mut img = initialize_blank_image(768, 1024);

    // Draw minimal elements vertices.
    let mut spacing: i32 = 768 / (minimal_elements.len() + 1) as i32;
    for mut elem in minimal_elements {
        draw_vertex(&mut img, spacing, 960, &mut elem);
        spacing += spacing;
    }

    let mut y = 900;
    for mut elem in set {
        draw_vertex(&mut img, 384, y, &mut elem);
        y -= 100;
    }

    // Save image
    img.save("test.jpg").expect("Failed to save image.");
}
