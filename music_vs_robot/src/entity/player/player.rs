use crate::entity::damage::DamagePlayer;
use crate::util::visitor::Visitable;

pub trait Player: Visitable {
    fn get_health(&self) -> u32;
    fn set_health(&mut self, health: u32);
    fn get_level(&self) -> u32;
    fn level_up(&mut self);
    fn get_cost(&self) -> u32;
    fn attack(&self) -> DamagePlayer;

    fn suffer_damage(&mut self, damage: u32) -> bool {
        if damage > self.get_health() {
            self.set_health(0);
            true
        } else {
            self.set_health(self.get_health() - damage);
            false
        }
    }
}
