mod ast;
mod context;
mod state;
use crate::parser;
use context::Context;
use state::{InState, OutState};
use std::collections::{HashSet, VecDeque};

fn explicate_open(
    operations: &Vec<parser::ast::Operation>,
    state: InState,
    context: &Context,
) -> Option<OutState> {
    // we use the state allocation for tracking things left to allocate
    let mut allocated = state.allocated.clone();
    let prev_fill = state.has_fill; // retain if this is the top-most "blob"
    return match schedule_operations(operations, state.incremented(true), context) {
        None => None,
        Some(mut result) => {
            // Run a simple BFS to remove remaining dependencies
            dbg!(&result.to_fill);
            let mut remaining = VecDeque::new();
            std::mem::swap(&mut remaining, &mut result.to_fill);
            // if the top-most blob, then fill in the rest
            if !prev_fill {
                let initial_result: HashSet<&String> = result.to_fill.iter().collect();
                remaining.append(
                    &mut context
                        .vars()
                        .iter()
                        .cloned()
                        .filter(|var| {
                            !(result.allocated.contains(var) || initial_result.contains(var))
                        })
                        .collect(),
                )
            }
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
                    .map(|v| v.iter().all(|s| allocated.contains(s)))
                    .unwrap_or(true)
                {
                    allocated.insert(element.clone());
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
            // eh, whatever
            while to_fill.len() > 0 {
                result
                    .ast
                    .operations
                    .push_front(ast::ScheduledOperation::Allocation(to_fill.pop().unwrap()));
            }
            Some(result)
        }
    };
}

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
            return explicate_open(operations, state, context);
        }
        parser::ast::Allocation::Type(typ) => context.get_matching(typ),
        parser::ast::Allocation::Var(v) => {
            temp.push(v.clone());
            &temp
        }
    };
    // attempt without any filling
    for var in options {
        if state.allocated.contains(var) {
            continue;
        }
        // if any dependencies are unallocated, skip
        if context
            .program
            .dependencies
            .get(var)
            .map(|v| v.iter().any(|x| !state.allocated.contains(x)))
            .unwrap_or(false)
        {
            continue;
        }
        let new_state = state.clone_alloc(var.clone());
        match schedule_operations(operations, new_state.incremented(false), context) {
            Some(mut result) => {
                result
                    .ast
                    .operations
                    .push_front(ast::ScheduledOperation::Allocation(var.clone()));
                return Some(result);
            }
            None => {}
        }
    }
    // Now try with filling
    for var in options {
        if state.allocated.contains(var) {
            continue;
        }
        let new_state = state.clone_alloc(var.clone());
        match schedule_operations(operations, new_state.incremented(false), context) {
            Some(mut result) => {
                let initial_result: HashSet<&String> = result.to_fill.iter().collect();
                result.to_fill.append(
                    &mut context
                        .program
                        .dependencies
                        .get(var)
                        .map(|v| {
                            v.iter()
                                .cloned()
                                .filter(|var| !initial_result.contains(var))
                                .collect()
                        })
                        .unwrap_or_else(|| VecDeque::new()),
                );
                result
                    .ast
                    .operations
                    .push_front(ast::ScheduledOperation::Allocation(var.clone()));
                return Some(result);
            }
            None => {}
        }
    }
    None
}

fn schedule_operations(
    operations: &Vec<parser::ast::Operation>,
    state: InState,
    context: &Context,
) -> Option<OutState> {
    match operations.get(state.index) {
        None => Some(OutState::new(state)),
        Some(parser::ast::Operation::Branch(_)) => todo!(),
        Some(parser::ast::Operation::Allocation(allocation)) => {
            schedule_allocation(operations, state, allocation, context)
        }
    }
}

pub fn schedule(program: parser::ast::Program) -> Option<ast::ScheduledProgram> {
    match schedule_operations(&program.operations, InState::new(), &Context::new(&program)) {
        None => None,
        Some(result) => {
            if result.to_fill.len() > 0 {
                None
            } else {
                Some(result.ast)
            }
        }
    }
}
