use crate::entity::damage::DamagePlayer;
use crate::entity::player::{consts::*, player::Player};
use crate::util::visitor::{Visitable, Visitor};

pub struct SlowDown {
    health: u32,
    level: u32,
}

impl SlowDown {
    pub fn new(level: u32) -> Self {
        Self {
            health: SLOW_DOWN_HEALTH + SLOW_DOWN_HEALTH_INCREMENT * level,
            level,
        }
    }
}

impl Visitable for SlowDown {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_slow_down();
    }
}

impl Player for SlowDown {
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
        self.health += SLOW_DOWN_HEALTH_INCREMENT;
    }

    fn get_cost(&self) -> u32 {
        SLOW_DOWN_COST * (self.level as f32 - 0.75) as u32
    }

    fn attack(&self) -> DamagePlayer {
        DamagePlayer::new(self.level * SLOW_DOWN_DAMAGE, 1 + self.get_level() / 2, 0)
    }
}