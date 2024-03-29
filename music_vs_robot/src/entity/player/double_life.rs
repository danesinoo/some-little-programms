use crate::entity::damage::DamagePlayer;
use crate::entity::player::{consts::*, player::Player};
use crate::util::visitor::{Visitable, Visitor};

pub struct DoubleLife {
    health: usize,
    level: usize,
    life: bool,
}

impl DoubleLife {
    pub fn new(level: usize) -> Self {
        Self {
            health: DOUBLE_LIFE_HEALTH + DOUBLE_LIFE_HEALTH_INCREMENT * level,
            level,
            life: true,
        }
    }
}

impl Visitable for DoubleLife {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_double_life();
    }
}

impl Player for DoubleLife {
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
        self.health += DOUBLE_LIFE_HEALTH_INCREMENT;
    }

    fn get_cost(&self) -> usize {
        DOUBLE_LIFE_COST * (self.level as f32 - 0.75) as usize
    }

    fn attack(&self) -> DamagePlayer {
        DamagePlayer::new(self.level * DOUBLE_LIFE_DAMAGE, 0, 0)
    }

    fn suffer_damage(&mut self, damage: usize) -> bool {
        if damage > self.get_health() {
            if self.life {
                self.life = false;
                self.health = DOUBLE_LIFE_HEALTH + DOUBLE_LIFE_HEALTH_INCREMENT * self.level;
                false
            } else {
                true
            }
        } else {
            self.set_health(self.get_health() - damage);
            false
        }
    }
}
