use crate::rdoc::Test;

use std::fmt::Display;

#[derive(Debug)]
pub struct Suite {
    location: String,
    tests: Vec<Test>,
}

impl Suite {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tests: vec![],
        }
    }

    pub fn location(&self) -> &String {
        &self.location
    }

    pub fn tests(&self) -> &Vec<Test> {
        &self.tests
    }

    pub fn push(&mut self, test: Test) {
        self.tests.push(test);
    }

    pub fn sort(&mut self) {
        self.tests.sort();
    }
}

impl Display for Suite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.location())
    }
}
