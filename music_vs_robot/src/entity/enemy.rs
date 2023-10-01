use crate::entity::damage::{Damage, DamagePlayer};
use crate::util::visitor::{Visitable, Visitor};

pub trait Enemy: Visitable {
    fn suffer_damage(&mut self, damage: &mut DamagePlayer) -> bool;
    fn attack(&mut self) -> usize;
    fn step(&mut self) -> usize;
    fn value(&self) -> usize;
    fn is_dead(&self) -> bool;
    fn copy(&self) -> Box<dyn Enemy>;
}

#[derive(Clone)]
pub struct EnemySimple {
    life: usize,
    attack: usize,
    step: usize,
    value: usize,
}

impl EnemySimple {
    pub fn new(life: usize, attack: usize, step: usize, value: usize) -> Self {
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

    fn attack(&mut self) -> usize {
        self.attack
    }

    fn step(&mut self) -> usize {
        self.step
    }

    fn value(&self) -> usize {
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
    life: usize,
    attack: usize,
    step: usize,
    value: usize,
}

impl EnemyDefense {
    pub fn new(life: usize, attack: usize, step: usize, value: usize) -> Self {
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

    fn attack(&mut self) -> usize {
        self.attack
    }

    fn step(&mut self) -> usize {
        self.step
    }

    fn value(&self) -> usize {
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
    life: usize,
    attack: usize,
    step: usize,
    value: usize,
}

impl EnemyBig {
    pub fn new(life: usize, attack: usize, step: usize, value: usize) -> Self {
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

    fn attack(&mut self) -> usize {
        self.attack
    }

    fn step(&mut self) -> usize {
        self.step
    }

    fn value(&self) -> usize {
        self.value
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn copy(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}
