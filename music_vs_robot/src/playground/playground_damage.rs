use crate::entity::{damage::*, enemy::Enemy, enemy_w_tool::EnemyWTool};
use crate::playground::{consts::COL, consts::ROW, playground::Playground};
use crate::util::observer_playground::{ObservablePlayground, ObserverPlayground};
use std::collections::VecDeque;

pub trait PlaygroundDamage<PE>: Playground<DamagePlayer, DamagePlayer>
where
    PE: Playground<EnemyWTool, VecDeque<EnemyWTool>> + ?Sized,
{
    fn attack(&mut self, pe: &mut PE, row: usize) {
        let mut col = 0;
        while col < self.cols() {
            let damage = self.get_mut(row, col);
            let cell = pe
                .get_mut(row, col)
                .iter_mut()
                .map(|x| {
                    x.suffer_damage(damage);
                    x
                })
                .filter(|x| x.is_dead())
                .map(|x| x.clone())
                .collect::<VecDeque<EnemyWTool>>();

            // I need to allocate the cell to a new variable because I can't borrow the cell
            // twice (is there any work around?)
            Playground::remove(pe, &cell, row, col);
            col += 1;
        }
    }

    fn move_(&mut self, pe: &mut PE, row: usize) {
        *self.get_mut(row, self.cols() - 1) = DamagePlayer::default();
        let mut col = self.cols() - 2;
        while col > 0 {
            if self.is_empty(row, col) {
                continue;
            }
            self.get_mut(row, col + 1).one_wave();

            let mut d = *self.get(row, col);
            if pe.is_empty(row, col) {
                Playground::remove(self, &d, row, col);
            } else {
                d.reset_slow();
                *self.get_mut(row, col) =
                    *self.get(row, col) / DamagePlayer::new(std::usize::MAX, 1, std::usize::MAX);
            }
            self.insert(row, col + 1, &d);
            col -= 1;
        }
    }

    fn is_slow(&mut self, row: usize, col: usize) -> usize {
        self.get_mut(row, col).slow()
    }
}

// Implement PlaygroundDamage
struct PlaygroundDamageImpl {
    playground: [[DamagePlayer; COL]; ROW],
    obs: Vec<Box<dyn ObserverPlayground<DamagePlayer>>>,
}

impl PlaygroundDamageImpl {
    fn new() -> Self {
        Self {
            playground: [[DamagePlayer::default(); COL]; ROW],
            obs: Vec::new(),
        }
    }
}

impl ObservablePlayground<DamagePlayer> for PlaygroundDamageImpl {
    fn register(&mut self, obs: Box<dyn ObserverPlayground<DamagePlayer>>) {
        self.obs.push(obs);
    }

    fn notify(&mut self, row: usize, col: usize, value: Option<&DamagePlayer>) {
        self.obs.iter_mut().for_each(|x| x.update(row, col, value));
    }
}

impl Playground<DamagePlayer, DamagePlayer> for PlaygroundDamageImpl {
    fn clear(&mut self) {
        for row in 0..ROW {
            for col in 0..COL {
                self.playground[row][col] = DamagePlayer::default();
            }
        }
    }

    fn is_empty(&self, row: usize, col: usize) -> bool {
        self.playground[row][col] != DamagePlayer::default()
    }

    fn insert(&mut self, row: usize, col: usize, value: &DamagePlayer) -> bool {
        if self.is_empty(row, col) {
            self.playground[row][col] = *value;
            self.notify(row, col, Some(value));
            true
        } else {
            false
        }
    }

    fn remove(&mut self, _value: &DamagePlayer, row: usize, col: usize) -> bool {
        if !self.is_empty(row, col) {
            self.playground[row][col] = DamagePlayer::default();
            self.notify(row, col, None);
            true
        } else {
            false
        }
    }

    fn get(&self, row: usize, col: usize) -> &DamagePlayer {
        &self.playground[row][col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut DamagePlayer {
        &mut self.playground[row][col]
    }

    fn cols(&self) -> usize {
        COL
    }
}

impl PlaygroundDamage<PlaygroundEnemyImpl> for PlaygroundDamageImpl {}
