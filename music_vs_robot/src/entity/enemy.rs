use crate::entity::damage::DamagePlayer;
use crate::util::visitor::{Visitable, Visitor};

trait Enemy: Visitable {
    fn suffer_damage(&mut self, damage: DamagePlayer) -> bool;
    fn attack(&self) -> u32;
    fn step(&self) -> u32;
}

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
    fn suffer_damage(&mut self, damage: DamagePlayer) -> bool {
        todo!()
        // need to implement cash
    }

    fn attack(&self) -> u32 {
        self.attack
    }

    fn step(&self) -> u32 {
        self.step
    }
}

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
    fn suffer_damage(&mut self, damage: DamagePlayer) -> bool {
        todo!()
        // need to implement cash
    }

    fn attack(&self) -> u32 {
        self.attack
    }

    fn step(&self) -> u32 {
        self.step
    }
}

struct EnemyBig {
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
    fn suffer_damage(&mut self, damage: DamagePlayer) -> bool {
        todo!()
        // need to implement cash
    }

    fn attack(&self) -> u32 {
        self.attack
    }

    fn step(&self) -> u32 {
        self.step
    }
}
