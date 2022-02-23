use std::fmt::Display;

pub enum Node {
    Operation {
        op: Op,
        left_operand: Box<Node>,
        right_operand: Box<Node>,
    },
    Value(f64),
}

impl Node {
    pub fn depth(&self, count: u32) -> u32 {
        match self {
            Node::Operation { op: _, left_operand, right_operand } => {
                let left_depth = left_operand.depth(count + 1);
                let right_depth = right_operand.depth(count + 1);
                if left_depth > right_depth {
                    left_depth
                } else {
                    right_depth
                }
            },
            Node::Value(_) => count + 1,
        }
    }
}

// impl Display for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
//     }
// }

pub enum Op {
    Add,
    Mul,
    Sub,
    Div,
}