use std::collections::HashMap;
use dialoguer::{Select, MultiSelect};
use std::io;
use std::io::Write;
use crate::set_util::{Matrix, Element};

pub struct Set {
    pub elements: Vec<Element>,
    pub idx: HashMap<u32, usize>
}

impl Set {
    pub fn new() -> Self {
        let mut set = Vec::new();
        let mut i = 1;

        // Get elements from user.
        println!("Enter elements in set:");
        loop {
            let mut input = String::new();
            print!("Enter member {}: ", i);
            io::stdout().flush().expect("Failed to flush.");
            io::stdin().read_line(&mut input).expect("Need a number.");

            if input == "\n" {
                break;
            }
            let element = Element::new(input);
            set.push(element.clone());
            i += 1;
        }

        // Sort set in ascending order
        set.sort_by(|a, b| a.value.cmp(&b.value));

        // Initialize element indices
        let mut idx = HashMap::new();
        for i in 0..set.len() {
            idx.insert(set[i].value, i);
        }

        Self {
            elements: set,
            idx: idx
        }
    }
}

pub struct Relation {
    pub pairs: Vec<(u32, u32)>,
    pub matrix: Matrix
}

impl Relation {
    pub fn build_from_set(set: &Set) -> Self {
        let choices: [&str; 3] = [
            "A <= B",
            "A | B",
            "Custom"
        ];
        // Prompt user to select type of relation
        let rel_select = Select::new()
            .with_prompt("Select type of relation")
            .items(&choices)
            .interact()
            .unwrap();

        // Build vector of all possible ordered pairs
        // for a menu display in the console.
        let mut pair_strings = Vec::new();
        let mut pair_elements = Vec::new();
        for a in 0..set.elements.len() {
            for b in a..set.elements.len() {
                pair_elements.push((set.elements[a].clone(), set.elements[b].clone()));

                let pair: String = format!("({}, {})", set.elements[a].value, set.elements[b].value);

                if rel_select == 0 {
                    pair_strings.push((pair, true));
                    continue;
                }

                if rel_select == 1 {
                    match set.elements[b].value % set.elements[a].value == 0 {
                        true => pair_strings.push((pair, true)),
                        false => pair_strings.push((pair, false))
                    }
                    continue;
                }

                match set.elements[a].value == set.elements[b].value {
                    true => pair_strings.push((pair, true)),
                    false => pair_strings.push((pair, false))
                } 
            }
        }

        // Prompt user to custom select the ordered pairs
        // in the relation.
        let selection = MultiSelect::new()
            .with_prompt("Build the relation")
            .items_checked(&pair_strings)
            .interact()
            .unwrap();

        // Process user selections and organize values
        // into a vector of tuples and a matrix.
        let mut relation = Vec::new();
        let mut matrix = Matrix::new(set.elements.len());
        for i in selection {
            let a = set.idx.get(&pair_elements[i].0.value);
            let b = set.idx.get(&pair_elements[i].1.value);
            relation.push((pair_elements[i].0.value, pair_elements[i].1.value));
            matrix.set_true(*a.unwrap(), *b.unwrap()); 
        }

        matrix.remove_transitivity();

        Self {
            pairs: relation,
            matrix: matrix
        }
    }
}
