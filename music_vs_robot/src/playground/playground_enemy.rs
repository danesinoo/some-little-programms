use crate::entity::{damage::*, enemy::Enemy, enemy_w_tool::EnemyWTool, player::player::Player};
use crate::playground::{consts::COL, consts::ROW, playground::Playground};
use crate::util::observer_playground::{ObservablePlayground, ObserverPlayground};
use std::collections::VecDeque;

trait PlaygroundEnemy<PP, PD>: Playground<EnemyWTool, VecDeque<EnemyWTool>>
where
    PP: Playground<Player, Player>,
    PD: Playground<DamagePlayer, DamagePlayer>,
{
    fn attack(&mut self, pp: &mut PP, row: usize);
    fn move_(&mut self, pp: &mut PP, pd: &mut PD, row: usize);
}
