use crate::entity::damage::DamagePlayer;
use crate::entity::tool::tool::Tool;
use crate::util::visitor::{Visitable, Visitor};

#[derive(Clone)]
pub struct Armor {
    durability: usize,
}

impl Armor {
    pub fn new(durability: usize) -> Self {
        Self { durability }
    }
}

impl Visitable for Armor {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_tool_armor();
    }
}

impl Tool for Armor {
    fn is_broken(&self) -> bool {
        self.durability == 0
    }

    fn suffer_damage(&mut self, damage: &mut DamagePlayer) {
        if !self.is_broken() {
            self.durability -= 1;
            *damage = DamagePlayer::new(0, 0, 0);
        }
    }

    fn copy(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }
}
