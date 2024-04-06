//use std::collections::VecDeque;

mod draw;
use crate::draw::*;

mod set;
use crate::set::*;

fn main() {

    // Initialize set vector with user input.
    let mut set = ElementVector::new();
    set.elements.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Initialize relation struct with user input.
    let relation = Relation::build_from_set(&set.elements);

    // Find minimal elements in relation.
    set.find_minimal_elements(&relation);

    // Initialize image.
    let mut img = initialize_blank_image(768, 1024);

    // Draw minimal elements vertices.
    let mut spacing: i32 = 768 / (set.minimal.len() + 1) as i32;
    for i in 0.. set.minimal.len() {
        draw_vertex(&mut img, spacing, 960, &mut set.minimal[i]);
        spacing += spacing;
    }

    for i in 0..set.minimal.len() {
        println!("{:?}", relation.map.get(&set.minimal[i].value));
    }

    let mut y = 900;
    for mut elem in set.elements {
        draw_vertex(&mut img, 384, y, &mut elem);
        y -= 100;
    }

    // Save image
    img.save("test.jpg").expect("Failed to save image.");
}
