use crate::entity::consts::{MAX_STEP, MIN_STEP};
use crate::entity::damage::DamagePlayer;
use crate::entity::enemy::*;
use crate::entity::tool::{armor::Armor, boots::Boots, ring::Ring, tool::Tool, weapon::Weapon};
use crate::util::visitor::{Visitable, Visitor};
use rand::Rng;

pub struct EnemyWTool {
    enemy: Box<dyn Enemy>,
    tool: Box<dyn Tool>,
}

impl EnemyWTool {
    pub fn new(min: u32, max: u32) -> Self {
        EnemyWTool {
            enemy: get_enemy(min, max),
            tool: get_tool(min, max),
        }
    }
}

fn get_enemy(min: u32, max: u32) -> Box<dyn Enemy> {
    let life = rand(min, max);
    let attack = rand(min, max);
    let step = rand(MIN_STEP, MAX_STEP);
    let value = rand(min, max);

    match rand(0, 2) {
        0 => Box::new(EnemySimple::new(life, attack, step, value)),
        1 => Box::new(EnemyDefense::new(life, attack, step, value)),
        2 => Box::new(EnemyBig::new(life, attack, step, value)),
        _ => unreachable!(),
    }
}

fn get_tool(min: u32, max: u32) -> Box<dyn Tool> {
    let durability = rand(0, 5);

    match rand(0, 3) {
        0 => Box::new(Armor::new(durability)),
        1 => Box::new(Boots::new(durability, rand(MIN_STEP, MAX_STEP))),
        2 => Box::new(Ring::new(rand(min, max))),
        3 => Box::new(Weapon::new(durability, rand(min, max))),
        _ => unreachable!(),
    }
}

#[inline]
fn rand(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}

impl Visitable for EnemyWTool {
    fn accept(&self, visitor: &mut dyn Visitor) {
        self.tool.accept(visitor);
        self.enemy.accept(visitor);
    }
}

impl Clone for EnemyWTool {
    fn clone(&self) -> Self {
        EnemyWTool {
            enemy: self.enemy.copy(),
            tool: self.tool.copy(),
        }
    }
}

impl Enemy for EnemyWTool {
    fn suffer_damage(&mut self, damage: &mut DamagePlayer) -> bool {
        self.tool.suffer_damage(damage);
        if self.enemy.suffer_damage(damage) {
            true
        } else {
            false
        }
    }

    fn attack(&mut self) -> u32 {
        self.enemy.attack() + self.tool.attack()
    }

    fn step(&mut self) -> u32 {
        self.enemy.step() + self.tool.step()
    }

    fn value(&self) -> u32 {
        self.enemy.value() + self.tool.value()
    }

    fn is_dead(&self) -> bool {
        self.enemy.is_dead()
    }

    fn copy(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}
