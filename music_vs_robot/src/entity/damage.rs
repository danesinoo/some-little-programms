use crate::util::visitor::{Visitable, Visitor};
use std::ops::{Add, Div};

pub trait Damage: Add + Div + Visitable + Copy + Sized + PartialEq + Default {
    fn damage(&mut self) -> u32;
    fn reset_slow(&mut self);
    fn slow(&mut self) -> bool;
    fn one_wave(&mut self);
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DamagePlayer {
    damage: u32,
    slow: u32,
    wave: u32,
    column: i8,
}

impl DamagePlayer {
    pub fn new(damage: u32, slow: u32, wave: u32) -> Self {
        DamagePlayer {
            damage,
            slow,
            wave,
            column: 4,
        }
    }
}

impl Add for DamagePlayer {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        DamagePlayer {
            damage: self.damage + other.damage,
            slow: self.slow + other.slow,
            wave: self.wave + other.wave,
            column: self.column,
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
            column: self.column,
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
        if self.slow > 0 {
            self.slow -= 1;
            true
        } else {
            false
        }
    }

    fn one_wave(&mut self) {
        self.column -= 1;
        if self.column == 0 {
            self.wave = 0;
        }
    }
}
