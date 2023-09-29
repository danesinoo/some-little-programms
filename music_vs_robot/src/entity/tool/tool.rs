use crate::entity::damage::DamagePlayer;
use crate::util::visitor::Visitable;

pub trait Tool: Visitable {
    fn is_broken(&self) -> bool;

    fn attack(&mut self) -> u32 {
        0
    }

    fn suffer_damage(&mut self, _damage: &mut DamagePlayer) {}

    fn step(&mut self) -> u32 {
        0
    }

    fn value(&self) -> u32 {
        0
    }
}
