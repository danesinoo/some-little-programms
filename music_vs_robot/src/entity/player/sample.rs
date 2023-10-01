use crate::entity::damage::DamagePlayer;
use crate::entity::player::{consts::*, player::Player};
use crate::util::visitor::{Visitable, Visitor};

pub struct PlayerSample {
    health: usize,
    level: usize,
}

impl PlayerSample {
    pub fn new(level: usize) -> Self {
        Self {
            health: SAMPLE_HEALTH + SAMPLE_HEALTH_INCREMENT * level,
            level,
        }
    }
}

impl Visitable for PlayerSample {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_sample();
    }
}

impl Player for PlayerSample {
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
        self.health += SAMPLE_HEALTH_INCREMENT;
    }

    fn get_cost(&self) -> usize {
        SAMPLE_COST * (self.level as f32 - 0.75) as usize
    }

    fn attack(&self) -> DamagePlayer {
        DamagePlayer::new(self.level * SAMPLE_DAMAGE, 0, 0)
    }
}
