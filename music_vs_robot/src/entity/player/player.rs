use crate::entity::damage::DamagePlayer;
use crate::util::visitor::Visitable;

pub trait Player: Visitable {
    fn get_health(&self) -> usize;
    fn set_health(&mut self, health: usize);
    fn get_level(&self) -> usize;
    fn level_up(&mut self);
    fn get_cost(&self) -> usize;
    fn attack(&self) -> DamagePlayer;

    fn suffer_damage(&mut self, damage: usize) -> bool {
        if damage > self.get_health() {
            self.set_health(0);
            true
        } else {
            self.set_health(self.get_health() - damage);
            false
        }
    }
}
