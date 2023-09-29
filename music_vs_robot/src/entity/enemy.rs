use crate::entity::damage::{Damage, DamagePlayer};
use crate::util::visitor::{Visitable, Visitor};

pub trait Enemy: Visitable {
    fn suffer_damage(&mut self, damage: &mut DamagePlayer) -> bool;
    fn attack(&mut self) -> u32;
    fn step(&mut self) -> u32;
    fn value(&self) -> u32;
    fn is_dead(&self) -> bool;
    fn copy(&self) -> Box<dyn Enemy>;
}

#[derive(Clone)]
pub struct EnemySimple {
    life: u32,
    attack: u32,
    step: u32,
    value: u32,
}

impl EnemySimple {
    pub fn new(life: u32, attack: u32, step: u32, value: u32) -> Self {
        EnemySimple {
            life,
            attack,
            step,
            value,
        }
    }
}

impl Visitable for EnemySimple {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_enemy();
    }
}

impl Enemy for EnemySimple {
    fn suffer_damage(&mut self, damage: &mut DamagePlayer) -> bool {
        let d = damage.damage();
        if d > self.life {
            self.life = 0;
            true
        } else {
            self.life -= d;
            false
        }
    }

    fn attack(&mut self) -> u32 {
        self.attack
    }

    fn step(&mut self) -> u32 {
        self.step
    }

    fn value(&self) -> u32 {
        self.value
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn copy(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct EnemyDefense {
    life: u32,
    attack: u32,
    step: u32,
    value: u32,
}

impl EnemyDefense {
    pub fn new(life: u32, attack: u32, step: u32, value: u32) -> Self {
        EnemyDefense {
            life,
            attack,
            step,
            value,
        }
    }
}

impl Visitable for EnemyDefense {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_enemy_defense();
    }
}

impl Enemy for EnemyDefense {
    fn suffer_damage(&mut self, damage: &mut DamagePlayer) -> bool {
        *damage = *damage / DamagePlayer::new(2, 1, 2);
        let d = damage.damage();
        if d > self.life {
            self.life = 0;
            true
        } else {
            self.life -= d;
            false
        }
    }

    fn attack(&mut self) -> u32 {
        self.attack
    }

    fn step(&mut self) -> u32 {
        self.step
    }

    fn value(&self) -> u32 {
        self.value
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn copy(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct EnemyBig {
    life: u32,
    attack: u32,
    step: u32,
    value: u32,
}

impl EnemyBig {
    pub fn new(life: u32, attack: u32, step: u32, value: u32) -> Self {
        EnemyBig {
            life,
            attack,
            step,
            value,
        }
    }
}

impl Visitable for EnemyBig {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_enemy_big();
    }
}

impl Enemy for EnemyBig {
    fn suffer_damage(&mut self, damage: &mut DamagePlayer) -> bool {
        let d = damage.damage();
        if d > self.life {
            self.life = 0;
            true
        } else {
            self.life -= d;
            false
        }
    }

    fn attack(&mut self) -> u32 {
        self.attack
    }

    fn step(&mut self) -> u32 {
        self.step
    }

    fn value(&self) -> u32 {
        self.value
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn copy(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}
