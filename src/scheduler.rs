mod ast;
mod context;
mod state;
use crate::parser;
use state::State;
use context::Context;
use std::collections::HashSet;

fn schedule_allocation(
    operations: &Vec<parser::ast::Operation>,
    index: usize,
    state: State,
    allocation: &parser::ast::Allocation,
    context: &Context,
) -> State {
    let mut temp = Vec::new();
    let options = match allocation {
        parser::ast::Allocation::Single => context.vars(),
        parser::ast::Allocation::Open => {
            result = schedule_operations(operations, index + 1, allocated, context);
            &temp
        }
        parser::ast::Allocation::Type(typ) => context.get_matching(typ),
        parser::ast::Allocation::Var(v) => {
            temp.push(v.clone());
            &temp
        }
    };
    for var in options {
        if allocated.contains(var) {
            continue;
        }
        // if any dependencies are unallocated, skip
        if match context.program.dependencies.get(var) {
            None => false,
            Some(v) => v.iter().any(|x| !allocated.contains(x)),
        } {
            continue;
        }
        let mut new_allocation = allocated.clone();
        new_allocation.insert(var.clone());
        match schedule_operations(operations, index + 1, new_allocation, context) {
            Some(mut result) => {
                result
                    .operations
                    .push(ast::ScheduledOperation::Allocation(var.clone()));
                return Some(result);
            }
            None => {}
        }
    }
    None
}

fn schedule_operations(
    operations: &Vec<parser::ast::Operation>,
    index: usize,
    allocated: HashSet<String>,
    context: &Context,
) -> Option<ast::ScheduledOperations> {
    match operations.get(index) {
        None => Some(ast::ScheduledOperations { operations: vec![] }),
        Some(parser::ast::Operation::Branch(_)) => todo!(),
        Some(parser::ast::Operation::Allocation(allocation)) => {
            schedule_allocation(operations, index, allocated, allocation, context)
        }
    }
}

pub fn schedule(program: parser::ast::Program) -> Option<ast::ScheduledProgram> {
    match schedule_operations(
        &program.operations,
        0,
        HashSet::new(),
        &Context::new(&program),
    ) {
        None => None,
        Some(mut result) => {
            result.operations.reverse();
            Some(result)
        }
    }
}
