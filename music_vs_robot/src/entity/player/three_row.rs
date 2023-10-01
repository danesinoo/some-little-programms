use crate::entity::damage::DamagePlayer;
use crate::entity::player::{consts::*, player::Player};
use crate::util::visitor::{Visitable, Visitor};

pub struct ThreeRow {
    health: usize,
    level: usize,
}

impl ThreeRow {
    pub fn new(level: usize) -> Self {
        Self {
            health: THREE_ROW_HEALTH + THREE_ROW_HEALTH_INCREMENT * level,
            level,
        }
    }
}

impl Visitable for ThreeRow {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_three_row();
    }
}

impl Player for ThreeRow {
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
        self.health += THREE_ROW_HEALTH_INCREMENT;
    }

    fn get_cost(&self) -> usize {
        THREE_ROW_COST * (self.level as f32 - 0.75) as usize
    }

    fn attack(&self) -> DamagePlayer {
        DamagePlayer::new(self.level * THREE_ROW_DAMAGE, 0, 0)
    }
}
