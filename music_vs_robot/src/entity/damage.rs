use crate::util::visitor::{Visitable, Visitor};
use std::ops::{Add, Div};

trait Damage: Add + Div + Visitable + Sized {
    fn damage(&mut self) -> u32;
    fn reset_slow(&mut self);
    fn slow(&mut self) -> bool;
    fn one_wave(&mut self);
}

pub struct DamagePlayer {
    damage: u32,
    slow: u32,
    wave: u32,
}

impl Add for DamagePlayer {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        DamagePlayer {
            damage: self.damage + other.damage,
            slow: self.slow + other.slow,
            wave: self.wave + other.wave,
        }
    }
}

impl Div for DamagePlayer {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        DamagePlayer {
            damage: self.damage / other.damage,
            slow: self.slow / other.slow,
            wave: self.wave / other.wave,
        }
    }
}

impl Visitable for DamagePlayer {
    fn accept(&self, visitor: &mut dyn Visitor) {
        if self.damage > 0 {
            visitor.visit_damage_bullet();
        }
        if self.slow > 0 {
            visitor.visit_damage_slow();
        }
        if self.wave > 0 {
            visitor.visit_damage_wave();
        }
    }
}

impl Damage for DamagePlayer {
    fn damage(&mut self) -> u32 {
        self.damage
    }

    fn reset_slow(&mut self) {
        self.slow = 0;
    }

    fn slow(&mut self) -> bool {
        self.slow > 0
    }

    fn one_wave(&mut self) {
        self.wave = 1;
    }
}
