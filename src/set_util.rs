#[derive(Clone, PartialEq, PartialOrd)]
pub struct Element {
    pub name: String,
    pub value: u32,
    pub coord: (f32, f32)
}

impl Element {
    pub fn new(elem: String) -> Self {
        Self {
            name: elem.clone(),
            value: elem.trim().parse().expect("NaN"),
            coord: (0.0, 0.0)
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

    pub fn size(&self) -> usize {
        self.num_columns
    }

    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        if col >= self.num_columns {
            return None;
        }
        let index  = self.num_columns * row + col;
        
        self.data.get(index).copied()
    }

    pub fn set_true(&mut self, row: usize, col: usize) {
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

    pub fn find_minimal_elements(&mut self) -> Vec<usize> {
        // For a minimal element u, if (x,u) in R, then u == x.
        let size = self.num_columns;
        let mut not_minimal = false;
        let mut min_elts = Vec::new();
        for col in 0..size {
            for row in 0..size {
                if self.get(row, col) == Some(false) && row == col {
                    not_minimal = true;
                    break;
                }
                if self.get(row, col) == Some(true) && row != col {
                    not_minimal = true;
                    break;
                }
            }

            if not_minimal == false {
                min_elts.push(col);
            }

            not_minimal = false;
        }

        min_elts
    }

    pub fn extract_minimal_elements(&mut self) ->Vec<usize> {
        let min_elts = self.find_minimal_elements();
        self.remove_minimal_elements(&min_elts);
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

    pub fn remove_transitivity(&mut self) {
        for row in 0..self.num_columns {            
            for col in (row+1..self.num_columns).rev() {
                if self.get(row, col) == Some(true) {
                    for c in row+1..col {
                        if self.get(row, c) == Some(true) && self.get(c, col) == Some(true) {
                            self.set_false(row, col);
                            break;
                        }
                    }
                }    
            }
        }
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
