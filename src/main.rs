mod set_util;

mod draw;
use crate::draw::*;

mod set;
use crate::set::*;

fn main() {

    // Initialize set vector with user input.
    let set = Set::new();

    // Initialize relation struct with user input.
    let relation = Relation::build_from_set(&set);

    // Initialize image.
    let mut img = initialize_blank_image(768, 1024);

    // Draw Hasse Diagram.
    let hasse_map = draw_hasse_diagram(&relation.matrix, &mut img);

    for i in hasse_map.iter() {
        println!("{:?}", i);
    }


//    for i in 0..set.minimal.len() {
//        println!("{:?}", relation.map.get(&set.minimal[i].value));
//    }

//    let mut y = 900;
//    for mut elem in set.elements {
//        draw_vertex(&mut img, 384, y, &mut elem);
//        y -= 100;
//    }

    // Save image
//    img.save("test.jpg").expect("Failed to save image.");
}
