use std::collections::HashMap;

pub type Type = String;
pub type Var = String;

#[derive(Debug, Clone)]
pub enum Allocation {
    Single, // ? -- one allocation of any type
    Open,   // ??? -- any number of allocations
    Type(Type), // an allocation of exactly type Type
    Var(Var)  // explicit allocation
}

#[derive(Debug, Clone)]
pub enum Operation {
    Allocation(Allocation),
    Scope(Vec<Operation>)
}

#[derive(Debug, Clone)]
pub struct Program {
    pub types: HashMap<Var, String>,
    pub dependencies: HashMap<Var, Vec<Var>>,
    pub allocations: Vec<Operation>
}