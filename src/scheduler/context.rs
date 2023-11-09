use std::collections::HashMap;

use crate::parser;
use crate::parser::ast::{Type, Var};

pub struct Context<'a> {
    pub program: &'a parser::ast::Program,
    typ_map: HashMap<Type, Vec<Var>>,
    // For fast iterating over the variables
    vars: Vec<Var>,
}

impl<'a> Context<'a> {
    pub fn new(program: &'a parser::ast::Program) -> Context<'a> {
        let mut typ_map = HashMap::new();
        let mut vars = Vec::new();
        for (var, typ) in &program.types {
            let entry = typ_map.entry(typ.clone()).or_insert_with(|| Vec::new());
            entry.push(var.clone());
            vars.push(var.clone());
        }
        Context {
            program,
            typ_map,
            vars,
        }
    }

    pub fn get_matching(&self, typ: &Type) -> &Vec<Var> {
        match self.typ_map.get(typ) {
            Some(v) => v,
            None => {
                panic!("Unknown type name {}", typ)
            }
        }
    }

    pub fn vars(&self) -> &Vec<Var> {
        &self.vars
    }
}
