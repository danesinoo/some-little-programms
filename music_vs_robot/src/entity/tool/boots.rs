use crate::entity::tool::tool::Tool;
use crate::util::visitor::{Visitable, Visitor};

#[derive(Clone)]
pub struct Boots {
    durability: usize,
    step: usize,
}

impl Boots {
    pub fn new(durability: usize, step: usize) -> Self {
        Self { durability, step }
    }
}

impl Visitable for Boots {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_tool_boots();
    }
}

impl Tool for Boots {
    fn is_broken(&self) -> bool {
        self.durability == 0
    }

    fn step(&mut self) -> usize {
        if self.is_broken() {
            0
        } else {
            self.durability -= 1;
            self.step
        }
    }

    fn copy(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }
}
