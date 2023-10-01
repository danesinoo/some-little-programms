use crate::entity::damage::DamagePlayer;
use crate::util::visitor::Visitable;

pub trait Tool: Visitable {
    fn is_broken(&self) -> bool;

    fn attack(&mut self) -> usize {
        0
    }

    fn suffer_damage(&mut self, _damage: &mut DamagePlayer) {}

    fn step(&mut self) -> usize {
        0
    }

    fn value(&self) -> usize {
        0
    }

    fn copy(&self) -> Box<dyn Tool>;
}
