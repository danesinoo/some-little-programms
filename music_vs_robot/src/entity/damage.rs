use crate::util::visitor::{Visitable, Visitor};
use std::ops::{Add, Div};

pub trait Damage: Add + Div + Visitable + Copy + Sized + PartialEq + Default {
    fn damage(&mut self) -> usize;
    fn reset_slow(&mut self);
    fn slow(&mut self) -> usize;
    fn one_wave(&mut self);
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DamagePlayer {
    damage: usize,
    slow: usize,
    wave: usize,
    column: i8,
}

impl DamagePlayer {
    pub fn new(damage: usize, slow: usize, wave: usize) -> Self {
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
    fn damage(&mut self) -> usize {
        self.damage
    }

    fn reset_slow(&mut self) {
        self.slow = 0;
    }

    fn slow(&mut self) -> usize {
        if self.slow > 0 {
            self.slow -= 1;
            1
        } else {
            0
        }
    }

    fn one_wave(&mut self) {
        self.column -= 1;
        if self.column == 0 {
            self.wave = 0;
        }
    }
}
