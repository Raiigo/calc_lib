#[derive(Debug)]
pub struct Node {
    pub content: Content,
    pub lhc: Option<Box<Node>>,
    pub rhc: Option<Box<Node>>,
}

impl Node {
    pub fn new(content: Content) -> Self {
        Self {
            content: content,
            lhc: None,
            rhc: None,
        }
    }

    pub fn put_l(&mut self, content: Content) {
        self.lhc = Some(Box::new(Node::new(content)));
    }
    pub fn put_r(&mut self, content: Content) {
        self.rhc = Some(Box::new(Node::new(content)));
    }
    pub fn get_l(&mut self) -> Option<&mut Box<Node>> {
        match &mut self.lhc {
            Some(v) => Some(v),
            None => None,
        }
    }
    pub fn get_r(&mut self) -> Option<&mut Box<Node>> {
        match &mut self.rhc {
            Some(v) => Some(v),
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum Content {
    Value(f64),
    Operation(Op)
}

#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
    Sub,
    Div,
}