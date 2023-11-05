pub type Var = String;
pub type Color = String;

pub struct Node {
    pub var : Var,
    pub colors: Vec<Color>
}

pub struct Graph {
    pub nodes: Vec<Node>
}