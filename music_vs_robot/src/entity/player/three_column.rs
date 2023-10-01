use crate::entity::damage::DamagePlayer;
use crate::entity::player::{consts::*, player::Player};
use crate::util::visitor::{Visitable, Visitor};

pub struct ThreeColumn {
    health: usize,
    level: usize,
}

impl ThreeColumn {
    pub fn new(level: usize) -> Self {
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
    fn get_health(&self) -> usize {
        self.health
    }

    fn set_health(&mut self, health: usize) {
        self.health = health;
    }

    fn get_level(&self) -> usize {
        self.level
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.health += THREE_COLUMN_HEALTH_INCREMENT;
    }

    fn get_cost(&self) -> usize {
        THREE_COLUMN_COST * (self.level as f32 - 0.75) as usize
    }

    fn attack(&self) -> DamagePlayer {
        DamagePlayer::new(0, 0, self.level * THREE_COLUMN_DAMAGE)
    }
}
