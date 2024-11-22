
use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if let Some(list) = self.grades.get_mut(&grade) {
            list.push(student.to_string());
        } else {
            self.grades.insert(grade, vec![student.to_string()]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut all_grades: Vec<u32> = Vec::new();
        for &key in self.grades.keys() {
            all_grades.push(key)
        }
        all_grades.sort();
        all_grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let result = self.grades.get(&grade);
        return match result {
            None => Vec::new(),
            Some(g) => {
                let mut tmp = g.to_vec().clone();
                tmp.sort();
                tmp
            }
        };
    }
}

fn main() {
    let mut school =  School::new();

    school.add(6,"Alice");
    school.add(6,"Bob");
    school.add(7, "Charlie");
    school.add(7, "David");
    school.add(8, "Eve");

    println!("All grades: {:?}", school.grades());
    println!("Students in grade 6: {:?}", school.grade(6));
    println!("Students in grade 7: {:?}", school.grade(7));
    println!("Students in grade 9: {:?}", school.grade(9));
}