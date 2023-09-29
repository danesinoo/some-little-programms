use crate::entity::tool::tool::Tool;
use crate::util::visitor::{Visitable, Visitor};

#[derive(Clone)]
pub struct Ring {
    value: u32,
}

impl Ring {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Visitable for Ring {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_tool_ring();
    }
}

impl Tool for Ring {
    fn is_broken(&self) -> bool {
        false
    }

    fn value(&self) -> u32 {
        self.value
    }

    fn copy(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }
}
