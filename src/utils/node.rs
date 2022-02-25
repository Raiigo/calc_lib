#[derive(Debug)]
pub struct Node {
    pub content: Content,
    lhc: Option<Box<Node>>,
    rhc: Option<Box<Node>>,
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
    pub fn depth(&mut self) -> u32 {
        let mut left_depth = 0;
        let mut right_depth = 0;
        if !self.get_l().is_none() {
            left_depth = self.get_l().unwrap().depth() + 1;
        }
        if !self.get_r().is_none() {
            right_depth = self.get_r().unwrap().depth() + 1;
        }
        if left_depth != 0 || right_depth != 0 {
            match left_depth < right_depth {
                true => return right_depth,
                false => return left_depth,
            }
        } else {
            return 1;
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