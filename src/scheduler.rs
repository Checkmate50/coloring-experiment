mod ast;
mod context;
mod state;
use crate::parser;
use context::Context;
use state::{InState, OutState};
use std::collections::VecDeque;

fn schedule_allocation(
    operations: &Vec<parser::ast::Operation>,
    state: InState,
    allocation: &parser::ast::Allocation,
    context: &Context,
) -> Option<OutState> {
    let mut temp = Vec::new();
    let options = match allocation {
        parser::ast::Allocation::Single => context.vars(),
        parser::ast::Allocation::Open => {
            return match schedule_operations(operations, state.incremented(), context) {
                None => None,
                Some(mut result) => {
                    let mut to_fill = Vec::new();
                    std::mem::swap(&mut to_fill, &mut result.to_fill);
                    result.ast.operations.extend(
                        to_fill
                            .into_iter()
                            // we need to reverse to undo the stack
                            .rev()
                            .map(|s| ast::ScheduledOperation::Allocation(s)),
                    );
                    Some(result)
                }
            };
        }
        parser::ast::Allocation::Type(typ) => context.get_matching(typ),
        parser::ast::Allocation::Var(v) => {
            temp.push(v.clone());
            &temp
        }
    };
    for var in options {
        if state.allocated.contains(var) {
            continue;
        }
        // if any dependencies are unallocated, skip
        if match context.program.dependencies.get(var) {
            None => false,
            Some(v) => v.iter().any(|x| !state.allocated.contains(x)),
        } {
            continue;
        }
        let new_state = state.clone_alloc(var.clone());
        match schedule_operations(operations, new_state.incremented(), context) {
            Some(mut result) => {
                result
                    .ast
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
    mut state: InState,
    context: &Context,
) -> Option<OutState> {
    match operations.get(state.index) {
        None => {
            // Run a simple BFS to remove remaining dependencies
            let mut remaining: VecDeque<String> = context
                .vars()
                .iter()
                .filter(|s| !state.allocated.contains(*s))
                .cloned()
                .collect();
            let mut to_fill = Vec::new();
            let mut count = 0;
            while remaining.len() > 0 {
                // get the first element of the queue
                let element = remaining.pop_front().unwrap();
                // if we have written all the dependencies for this element
                if context
                    .program
                    .dependencies
                    .get(&element)
                    .unwrap()
                    .iter()
                    .all(|s| state.allocated.contains(s))
                {
                    state.allocated.insert(element.clone());
                    to_fill.push(element);
                    count = 0;
                } else {
                    remaining.push_back(element);
                    // Safety check if we didn't make progress on the last loop
                    if count >= remaining.len() {
                        return None;
                    }
                    count += 1;
                }
            }
            Some(OutState::new(to_fill))
        }
        Some(parser::ast::Operation::Branch(_)) => todo!(),
        Some(parser::ast::Operation::Allocation(allocation)) => {
            schedule_allocation(operations, state, allocation, context)
        }
    }
}

pub fn schedule(program: parser::ast::Program) -> Option<ast::ScheduledProgram> {
    match schedule_operations(&program.operations, InState::new(), &Context::new(&program)) {
        None => None,
        Some(mut result) => {
            if result.to_fill.len() > 0 {
                None
            } else {
                result.ast.operations.reverse();
                Some(result.ast)
            }
        }
    }
}
