use std::collections::HashSet;

use crate::parser;

#[derive(Debug, Clone)]
pub struct State {
    open_operations: Vec<Vec<parser::ast::Var>>,
    pub allocated: HashSet<parser::ast::Var>,
}

impl State {
    pub fn new() -> State {
        State {
            open_operations: Vec::new(),
            allocated: HashSet::new(),
        }
    }

    pub fn push_open(&mut self) {
        self.open_operations.push(Vec::new())
    }

    pub fn can_add_open_allocation(&mut self, allocation : &str) -> bool {
        self.open_operations.len() > 0 && self.allocated.contains(allocation)
    }

    pub fn add_open_allocation(&mut self, allocation : String) {
        let check = self.allocated.insert(allocation.clone());
        assert!(check);
        self.open_operations.last_mut().unwrap().push(allocation)
    }    
}
