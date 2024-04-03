use std::collections::HashMap;
use dialoguer::MultiSelect;
use std::io;
use std::io::Write;

pub fn get_set() -> Vec<String> {
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

        set.push(input);
    }

    set
}

pub fn build_relation(set: &Vec<String>) -> Vec<(u32,u32)> { 
    let mut pairs = Vec::new();
    for a in 0..set.len() {
        for b in a..set.len() {
            let pair: String = format!("({}, {})", set[a].trim(), set[b].trim());
            pairs.push(pair);
        }
    }

    let selection = MultiSelect::new()
        .with_prompt("Build the relation")
        .items(&pairs)
        .interact()
        .unwrap();

    let mut relation = Vec::new();
    for i in selection {
        let pair: Vec<&str> = pairs[i]
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();
        let a = pair[0].trim().parse().expect("Failed to parse.");
        let b = pair[1].trim().parse().expect("Failed to parse.");
        relation.push((a,b));
    }

    relation

}

pub fn find_minimal_elements(
    set: &Vec<String>, 
    relation: &Vec<(u32,u32)>
    ) -> Vec<String> {
    // For a minimal element u, if (x,u) in R, then u == x.
    let mut min_elts: HashMap<u32, String> = HashMap::new();

    for elem in set.iter() {
        min_elts.insert(elem.trim().parse().expect("NaN"), elem.to_string());
    }

    for pair in relation {
        if pair.1 != pair.0 {
            min_elts.remove(&pair.1);
        }
    }


    print!("Minimal Elements: ");
    print!("{{ ");
    for value in min_elts.values() {
        print!("{} ", value.trim());
    }
    println!("}}");

    let result: Vec<String> = min_elts.values().cloned().collect();

    result
}
