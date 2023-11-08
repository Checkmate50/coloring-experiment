mod ast;
mod context;
use crate::parser;
use context::Context;
use std::collections::HashSet;

fn schedule_allocation(
    allocated: HashSet<String>,
    operation: parser::ast::Operation,
    context: &Context,
) -> Option<String> {
    match operation {
        parser::ast::Operation::Branch(_) => todo!(),
        parser::ast::Operation::Allocation(alloc) => match alloc {
            parser::ast::Allocation::Single => todo!(),
            parser::ast::Allocation::Open => todo!(),
            parser::ast::Allocation::Type(_) => todo!(),
            parser::ast::Allocation::Var(_) => todo!(),
        },
    }
}

fn schedule_operations(
    operations: &Vec<parser::ast::Operation>,
    index: usize,
    current: Vec<ast::ScheduledOperation>,
    allocated: HashSet<String>,
    context: &Context,
) -> Option<ast::ScheduledOperations> {
    match operations.get(index) {
        None => Some(ast::ScheduledOperations::new(current)),
        Some(parser::ast::Operation::Branch(_)) => todo!(),
        Some(parser::ast::Operation::Allocation(alloc)) => {}
    }
}

pub fn schedule(program: parser::ast::Program) -> Option<ast::ScheduledProgram> {
    schedule_operations(
        &program.operations,
        0,
        Vec::new(),
        HashSet::new(),
        &Context::new(&program),
    )
}
