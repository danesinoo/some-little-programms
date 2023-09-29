use crate::entity::damage::DamagePlayer;
use crate::entity::player::{consts::*, player::Player};
use crate::util::visitor::{Visitable, Visitor};

pub struct ThreeColumn {
    health: u32,
    level: u32,
}

impl ThreeColumn {
    pub fn new(level: u32) -> Self {
        Self {
            health: THREE_COLUMN_HEALTH + THREE_COLUMN_HEALTH_INCREMENT * level,
            level,
        }
    }
}

impl Visitable for ThreeColumn {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_three_column();
    }
}

impl Player for ThreeColumn {
    fn get_health(&self) -> u32 {
        self.health
    }

    fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    fn get_level(&self) -> u32 {
        self.level
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.health += THREE_COLUMN_HEALTH_INCREMENT;
    }

    fn get_cost(&self) -> u32 {
        THREE_COLUMN_COST * (self.level as f32 - 0.75) as u32
    }

    fn attack(&self) -> DamagePlayer {
        DamagePlayer::new(0, 0, self.level * THREE_COLUMN_DAMAGE)
    }
}
