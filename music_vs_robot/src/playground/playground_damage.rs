use crate::entity::{damage::DamagePlayer, enemy::Enemy, enemy_w_tool::EnemyWTool};
use crate::playground::playground::Playground;
use crate::util::observer_playground::{ObservablePlayground, ObserverPlayground};
use std::collections::VecDeque;

trait PlaygroundDamage<PE>: Playground<DamagePlayer, DamagePlayer>
where
    PE: Playground<EnemyWTool, VecDeque<EnemyWTool>>,
{
    fn attack(&mut self, pe: &mut PE, row: usize, col: usize) {
        if let Some(damage) = self.get_mut(row, col) {
            unsafe {
                let cell = pe.get_mut(row, col);
                cell.unwrap()
                    .iter_mut()
                    .map(|x| {
                        x.suffer_damage(damage);
                        x
                    })
                    .filter(|x| x.is_dead())
                    .for_each(|x| {
                        Playground::<EnemyWTool, VecDeque<EnemyWTool>>::remove(pe, x, row, col);
                    });
            }
        }
    }
}
