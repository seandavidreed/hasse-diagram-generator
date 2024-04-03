use std::collections::HashMap;
//use std::collections::VecDeque;

mod draw;
use crate::draw::*;

mod set;
use crate::set::*;

fn main() {

    // Initialize set with user input.
    let set: Vec<String> = get_set();

    // Initialize relation with user input.
    let relation: Vec<(u32,u32)> = build_relation(&set);

    // Find minimal elements in relation.
    let minimal_elements: Vec<String> = find_minimal_elements(&set, &relation);

    // Initialize image.
    let mut img = initialize_blank_image(768, 1024);

    // Draw minimal elements vertices.
    // Keep track of coordinates drawn.
    let mut vertex_locations: HashMap<String, (u32,u32)> = HashMap::new();

    let mut spacing: i32 = 768 / (minimal_elements.len() + 1) as i32;
    for elem in minimal_elements {
        draw_vertex(&mut img, spacing, 960, &elem);
        vertex_locations.insert(elem, (spacing as u32, 960));
        spacing += spacing;
    }

    // Keep track of adjacencies and which have yet to be traversed.
    //let mut adjacency_list: HashMap<&String, VecDeque<String>> = HashMap::new();
    //for elem in set.iter() {
    //    adjacency_list.insert(elem, VecDeque::new());    
    //}

    //for pair in relation.iter() {
    //    adjacency_list.get_mut(String::from(pair.0)).push_back(pair.1);
    //}

    let mut y = 900;
    for elem in set {
        draw_vertex(&mut img, 384, y, &elem);
        y -= 100;
    }

    for value in vertex_locations {
        println!("{:?}", value);
    }

    // Save image
    img.save("test.jpg").expect("Failed to save image.");
}
