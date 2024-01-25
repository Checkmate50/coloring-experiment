mod ast;
mod context;
mod state;
use crate::parser;
use context::Context;
use state::{InState, OutState};
use std::collections::{BTreeSet, VecDeque};

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
            let mut current = BTreeSet::new();
            std::mem::swap(&mut current, &mut result.to_fill);
            // if the top-most blob, then fill in the rest

            let mut remaining_fills: VecDeque<String> = if !prev_fill {
                context
                    .vars()
                    .iter()
                    .cloned()
                    .filter(|var| !(result.allocated.contains(var) || allocated.contains(var)))
                    .collect()
            } else {
                current
                    .iter()
                    .cloned()
                    .filter(|var| !(result.allocated.contains(var) || allocated.contains(var)))
                    .collect()
            };
            let mut to_fill = Vec::new();
            let mut count = 0;
            while remaining_fills.len() > 0 {
                // get the first element of the queue
                let element = remaining_fills.pop_front().unwrap();

                // if we have written all the dependencies for this element
                let deps = context.program.dependencies.get(&element);
                if deps
                    .map(|v| v.iter().all(|s| allocated.contains(s)))
                    .unwrap_or(true)
                {
                    allocated.insert(element.clone());
                    to_fill.push(element);
                    count = 0;
                }
                // if there are any elements we are planning to fill, wait
                else if deps
                    .map(|v| v.iter().any(|s| remaining_fills.contains(s)))
                    .unwrap_or(false)
                {
                    // cycle management, if there is a dependency cycle, we die
                    if count >= remaining_fills.len() {
                        return None;
                    }
                    count += 1;
                    remaining_fills.push_back(element);
                }
                // otherwise request resolution at a later time
                else {
                    allocated.insert(element.clone());
                    to_fill.push(element);
                    for item in deps.cloned().unwrap_or(Vec::new()) {
                        result.to_fill.insert(item);
                    }
                }
            }
            // eh, whatever
            to_fill
                .drain(..)
                .rev()
                .for_each(|element| result.allocate(element));
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
                result.allocate(var.clone());
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
                if let Some(data) = context.program.dependencies.get(var) {
                    result.to_fill.extend(&mut data.iter().cloned());
                }
                result.allocate(var.clone());
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
        None => Some(OutState::new()),
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
