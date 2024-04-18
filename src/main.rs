mod set_util;

mod draw;
use crate::draw::*;

mod set;
use crate::set::*;

fn main() {

    // Initialize set vector with user input.
    let mut set = Set::new();

    // Initialize relation struct with user input.
    let relation = Relation::build_from_set(&set);

    // Initialize image.
    let mut img = initialize_blank_image(768, 1024);

    // Draw Hasse Diagram on image.
    draw_hasse_diagram(&mut set, &relation.matrix, &mut img);

    // Save image.
    img.save("test.jpg").expect("Failed to save image.");
}
