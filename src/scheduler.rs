mod ast;
use crate::parser;

pub fn schedule(program: parser::ast::Program) -> ast::ScheduledProgram {
    ast::ScheduledProgram {
        allocations: vec![],
    }
}
