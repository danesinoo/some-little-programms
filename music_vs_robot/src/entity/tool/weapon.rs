use crate::entity::tool::tool::Tool;
use crate::util::visitor::{Visitable, Visitor};

#[derive(Clone)]
pub struct Weapon {
    damage: u32,
    durability: u32,
}

impl Weapon {
    pub fn new(durability: u32, damage: u32) -> Self {
        Self { damage, durability }
    }
}

impl Visitable for Weapon {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_tool_weapon();
    }
}

impl Tool for Weapon {
    fn is_broken(&self) -> bool {
        self.durability == 0
    }

    fn attack(&mut self) -> u32 {
        if self.is_broken() {
            0
        } else {
            self.durability -= 1;
            self.damage
        }
    }

    fn copy(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }
}
