use std::fmt;

pub enum ScheduledAllocation {
    Branch {
        left: ScheduledAllocations,
        right: ScheduledAllocations,
    },
    Allocation(String),
}

pub struct ScheduledAllocations {
    pub allocations: Vec<ScheduledAllocation>
}

pub type ScheduledProgram = ScheduledAllocations;

impl fmt::Display for ScheduledAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduledAllocation::Branch { left, right } =>
                write!(f, "if {{\n{}}}\nelse {{\n{}}}\n", left, right),
            ScheduledAllocation::Allocation(s) => write!(f, "{};\n", s),
        }
    }
}

impl fmt::Display for ScheduledAllocations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();
        for allocation in &self.allocations {
            result += &allocation.to_string()
        }
        write!(f, "{}", result)
    }
}