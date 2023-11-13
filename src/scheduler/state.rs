use std::collections::HashSet;

use crate::parser;
use crate::scheduler::ast;

#[derive(Debug, Clone)]
pub struct InState {
    pub allocated: HashSet<parser::ast::Var>,
    pub index: usize
}

impl InState {
    pub fn new() -> InState {
        InState {
            allocated: HashSet::new(),
            index: 0
        }
    }

    pub fn incremented(self) -> InState {
        InState {
            allocated: self.allocated,
            index: self.index + 1
        }
    }

    pub fn clone_alloc(&self, allocation: parser::ast::Var) -> InState {
        let mut allocated = self.allocated.clone();
        let check = allocated.insert(allocation);
        assert!(check);
        InState {
            allocated,
            index: self.index
        }
    }
}

#[derive(Debug)]
pub struct OutState {
    pub ast: ast::ScheduledOperations,
    // the operations to insert at the next "open" slot
    pub to_fill: Vec<String>,
}

impl OutState {
    pub fn new(to_fill: Vec<String>) -> OutState {
        OutState {
            ast: ast::ScheduledOperations::new(Vec::new()),
            to_fill,
        }
    }  
}