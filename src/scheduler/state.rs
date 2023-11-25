use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::parser;
use crate::scheduler::ast;

#[derive(Debug, Clone)]
pub struct InState {
    pub allocated: HashSet<parser::ast::Var>,
    pub index: usize,
    // whether or not an open slot is available
    // technically is an optimization, but whatever
    pub has_fill: bool,
}

impl InState {
    pub fn new() -> InState {
        InState {
            allocated: HashSet::new(),
            index: 0,
            has_fill: false,
        }
    }

    pub fn incremented(self, add_fill: bool) -> InState {
        InState {
            allocated: self.allocated,
            index: self.index + 1,
            has_fill: self.has_fill || add_fill,
        }
    }

    pub fn clone_alloc(&self, allocation: parser::ast::Var) -> InState {
        let mut allocated = self.allocated.clone();
        let check = allocated.insert(allocation);
        assert!(check);
        InState {
            allocated,
            index: self.index,
            has_fill: self.has_fill,
        }
    }
}

#[derive(Debug)]
pub struct OutState {
    pub ast: ast::ScheduledOperations,
    // the allocations to insert at the next "open" slot
    // order matters, hence BTreeSet vs HashSet
    pub to_fill: BTreeSet<parser::ast::Var>,
    // set of allocations worked out recursively, used to help with filling
    pub allocated: HashSet<parser::ast::Var>,
}

impl OutState {
    pub fn new(state: InState) -> OutState {
        OutState {
            ast: ast::ScheduledOperations::new(VecDeque::new()),
            to_fill: BTreeSet::new(),
            allocated: state.allocated,
        }
    }
}
