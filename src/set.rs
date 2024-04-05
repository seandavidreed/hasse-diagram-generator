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

pub struct Relation {
    pub pairs: Vec<(u32,u32)>,
    pub map: HashMap<u32, Vec<u32>>
}

impl Relation {
    pub fn build_from_set(set: &Vec<Element>) -> Self {
        let mut pairs = Vec::new();
        for a in 0..set.len() {
            for b in a..set.len() {
                let pair: String = format!("({}, {})", set[a].value, set[b].value);
                pairs.push(pair);
            }
        }

        let selection = MultiSelect::new()
            .with_prompt("Build the relation")
            .items(&pairs)
            .interact()
            .unwrap();

        let mut relation_map = HashMap::new();
        let mut relation = Vec::new();
        for i in selection {
            let pair: Vec<&str> = pairs[i]
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .collect();
            let a = pair[0].trim().parse().expect("Failed to parse.");
            let b = pair[1].trim().parse().expect("Failed to parse.");
            relation.push((a,b));
            relation_map.insert(a, Vec::new());
            if let Some(x) = relation_map.get_mut(&a) {
                x.push(b);
            }
        }

        Self {
            pairs: relation,
            map: relation_map
        }
    }
}

pub fn get_set() -> Vec<Element> {
    let mut set = Vec::new();
    let mut i = 1;

    println!("Enter elements in set:");

    loop {
        let mut input = String::new();

        print!("Enter member {}: ", i);
        i += 1;
        io::stdout().flush().expect("Failed to flush.");
        io::stdin().read_line(&mut input).expect("Need a number.");

        if input == "\n" {
            break;
        }

        set.push(Element::new(input));
    }

    set
}

pub fn find_minimal_elements(
    set: &Vec<Element>, 
    relation: &Relation
    ) -> Vec<Element> {
    // For a minimal element u, if (x,u) in R, then u == x.
    let mut min_elts = HashMap::new();

    for elem in set {
        min_elts.insert(elem.value, elem.clone()); // Like to find another way be
    }

    for pair in relation.pairs.iter() {
        if pair.1 != pair.0 {
            min_elts.remove(&pair.1);
        }
    }

    let mut result = Vec::new();
    print!("Minimal Elements: ");
    print!("{{ ");
    for elem in min_elts.values() {
        print!("{} ", elem.name.trim());
        result.push(elem.clone());
    }
    println!("}}");

    result
}
