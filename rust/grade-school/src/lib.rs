use std::collections::BTreeMap;
// quick and dirty way to ensure sort order
use std::collections::BTreeSet;
use std::iter::Iterator;

pub struct School {
    students: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students_list = self.students
            .entry(grade)
            .or_insert_with(BTreeSet::new);
        students_list.insert(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.students
            .get(&grade)
            .cloned()
            .map(BTreeSet::into_iter)
            .map(Iterator::collect)
    }
}
