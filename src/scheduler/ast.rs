use std::fmt;

pub enum ScheduledOperation {
    Branch {
        left: ScheduledOperations,
        right: ScheduledOperations,
    },
    Allocation(String),
}

pub struct ScheduledOperations {
    pub operations: Vec<ScheduledOperation>,
}

pub type ScheduledProgram = ScheduledOperations;

impl ScheduledOperations {
    pub fn new(operations: Vec<ScheduledOperation>) -> ScheduledOperations {
        ScheduledOperations { operations }
    }
}

impl fmt::Display for ScheduledOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduledOperation::Branch { left, right } => {
                write!(f, "if {{\n{}}}\nelse {{\n{}}}\n", left, right)
            }
            ScheduledOperation::Allocation(s) => write!(f, "{};\n", s),
        }
    }
}

impl fmt::Display for ScheduledOperations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();
        for operation in &self.operations {
            result += &operation.to_string()
        }
        write!(f, "{}", result)
    }
}
