use std::collections::HashMap;
use dialoguer::MultiSelect;
use std::io;
use std::io::Write;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Element {
    pub name: String,
    pub value: u32,
    pub coord: (u32, u32)
}

impl Element {
    pub fn new(elem: String) -> Self {
        Self {
            name: elem.clone(),
            value: elem.trim().parse().expect("NaN"),
            coord: (0, 0)
        }
    }
}

#[derive(Clone)]
pub struct Matrix {
    data: Vec<bool>,
    num_columns: usize
}

impl Matrix {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![false; size*size],
            num_columns: size
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        if col >= self.num_columns {
            return None;
        }
        let index  = self.num_columns * row + col;
        
        self.data.get(index).copied()
    }

    pub fn set_true(&mut self, row: usize, col: usize) {
        println!("{:?} {:?}", row, col);
        let index = self.num_columns * row + col;
        self.data[index] = true;
    }

    pub fn set_false(&mut self, row: usize, col: usize) {
        let index = self.num_columns * row + col;
        self.data[index] = false;
    }

    pub fn remove_minimal_elements(&mut self, min_elts: &Vec<usize>) {
        for min_elt in min_elts {
            for i in 0..self.num_columns {
                self.set_false(*min_elt, i);
            }
        }
    }

    fn find_minimal_elements(&mut self) -> Vec<usize> {
        // For a minimal element u, if (x,u) in R, then u == x.
        let size = self.num_columns;
        let mut not_minimal = false;
        let mut min_elts = Vec::new();
        for i in 0..size {
            for j in 0..size {
                if i == j {
                    continue;
                }
                if self.get(j, i) == Some(true) {
                    not_minimal = true;
                    break;
                }
            }

            if not_minimal == false {
                min_elts.push(i);
            }

            not_minimal = false;
        }

        min_elts
    }

    pub fn is_empty(&self) -> bool {
        for i in 0..self.num_columns {
            for j in 0..self.num_columns {
                let index = self.num_columns * i + j;
                if self.data[index] == true {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(&self) {
        print!("  ");
        for num in 0..self.num_columns {
            print!("{} ", num);
        }

        for (idx, value) in self.data.iter().enumerate() {
            if idx % self.num_columns == 0 {
                println!("");
                print!("{} ", idx / self.num_columns);
            }
            print!("{} ", *value as u8);
        }
        println!("");
        println!("");
    }
}

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

    pub fn build_hasse_map(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut map = HashMap::new();
        let mut prev_min_elts = Vec::new();
        let mut matrix_copy = self.matrix.clone();

        loop {
            matrix_copy.print();

            let min_elts = matrix_copy.find_minimal_elements();

            // Build hasse map
            for curr in min_elts.iter() {
                map.insert(*curr, Vec::new());
                for prev in prev_min_elts.iter() {
                    if self.matrix.get(*prev, *curr) == Some(true) {
                        map.get_mut(curr).expect("Key not in HashMap").push(*prev);
                    }
                }
            }
    
            matrix_copy.remove_minimal_elements(&min_elts);
            if matrix_copy.is_empty() {
                break;
            }

            prev_min_elts = min_elts.clone();
        }

        map
    }
}
