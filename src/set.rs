use std::collections::HashMap;
use dialoguer::MultiSelect;
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
        set.sort_by(|a, b| a.partial_cmp(b).unwrap());

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
        // Build vector of all possible ordered pairs
        // for a menu display in the console.
        let mut pair_strings = Vec::new();
        let mut pair_elements = Vec::new();
        for a in 0..set.elements.len() {
            for b in a..set.elements.len() {
                let pair: String = format!("({}, {})", set.elements[a].value, set.elements[b].value);
                pair_strings.push(pair);
 
                let pair: (Element, Element) = (set.elements[a].clone(), set.elements[b].clone());
                pair_elements.push(pair);
            }
        }
        
        // Prompt user to select the ordered pairs
        // in the relation.
        let selection = MultiSelect::new()
            .with_prompt("Build the relation")
            .items(&pair_strings)
            .interact()
            .unwrap();

        // Process user selections and organize values
        // into a vector of tuples and a matrix.
        let mut relation = Vec::new();
        let mut matrix = Matrix::new(set.elements.len());
        for i in selection {
//            let pair: Vec<&str> = pairs[i]
//                .trim_matches(|c| c == '(' || c == ')')
//                .split(',')
//                .collect();
//            let a = pair[0].trim().parse().expect("Failed to parse.");
//            let b = pair[1].trim().parse().expect("Failed to parse.");
            
            let a = set.idx.get(&pair_elements[i].0.value);
            let b = set.idx.get(&pair_elements[i].1.value);
            relation.push((pair_elements[i].0.value, pair_elements[i].1.value));
            matrix.set_true(*a.unwrap(), *b.unwrap()); 
        }

        Self {
            pairs: relation,
            matrix: matrix
        }
    }
}
