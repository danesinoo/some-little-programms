use crate::entity::{damage::*, enemy::Enemy, enemy_w_tool::EnemyWTool, player::player::Player};
use crate::playground::{consts::*, playground::Playground, playground_damage::PlaygroundDamage};
use crate::util::observer_playground::{ObservablePlayground, ObserverPlayground};
use std::collections::VecDeque;

trait PlaygroundEnemy<PP, PD, T>: Playground<EnemyWTool, VecDeque<EnemyWTool>>
where
    PP: Playground<T, T>,
    PD: PlaygroundDamage<Self>,
    T: Player,
{
    // in this case the col should be the product of the number of frames and the number of columns
    fn pop(&mut self, row: usize, col: usize) -> Option<EnemyWTool>;
    // in this case we use insert instead of push, which is defined in the trait Playground
    // fn push(&mut self, row: usize, col: usize, e: &EnemyWTool);

    fn attack(&mut self, pp: &mut PP, row: usize) {
        for col in 0..COL - 1 {
            if !pp.is_empty(row, col) {
                let d = self.get_mut(row, col).iter().map(|e| e.attack()).count() as usize;
                if pp.get_mut(row, col).suffer_damage(d) {
                    pp.remove(pp.get(row, col), row, col);
                }
            }
        }
    }

    fn move_(&mut self, pp: &mut PP, pd: &mut PD, row: usize) -> bool {
        for col in 1..COL * FRAMES {
            // check if there is a player in the current cell, if so the enemy doesn't move
            if !pp.is_empty(row, col / FRAMES) {
                col += FRAMES;
                continue;
            }

            let np = nearest_player(row, col, pp);

            // be aware that the enemy needs to move at least one cell
            while !self.is_empty(row, col) {
                let mut e = self.pop(row, col).unwrap();
                let mut mv = e.step() / (pd.is_slow(row, col / FRAMES) + 1);
                mv = col - std::cmp::min(mv, np);
                if mv == 0 {
                    self.insert(row, 0, &e);
                    self.notify(row, 0, Some(&e));
                    return true;
                }

                self.insert(row, mv, &e);
            }
        }
        for col in 0..COL {
            self.notify(row, col, None);
            self.get(row, col)
                .iter()
                .for_each(|e| self.notify(row, col, Some(e)));
        }

        false
    }
}

fn nearest_player<PP, T>(row: usize, col: usize, pp: &PP) -> usize
where
    PP: Playground<T, T>,
    T: Player,
{
    let mut c = (col - 1) / FRAMES;

    if !pp.is_empty(row, c) {
        return 0;
    }

    while c > 0 {
        if !pp.is_empty(row, c) {
            return col - c * FRAMES;
        }
        c -= 1;
    }

    col
}

// Implement PlaygroundEnemy
pub struct PlaygroundEnemyImpl {
    playground: [[VecDeque<EnemyWTool>; COL * FRAMES]; ROW],
    observers: Vec<Box<dyn ObserverPlayground<EnemyWTool>>>,
}

impl PlaygroundEnemyImpl {
    pub fn new() -> Self {
        PlaygroundEnemyImpl {
            playground: [[VecDeque::new(); COL * FRAMES]; ROW],
            observers: Vec::new(),
        }
    }
}

impl ObservablePlayground<EnemyWTool> for PlaygroundEnemyImpl {
    fn register(&mut self, observer: Box<dyn ObserverPlayground<EnemyWTool>>) {
        self.observers.push(observer);
    }

    fn notify(&mut self, row: usize, col: usize, e: Option<&EnemyWTool>) {
        for observer in &mut self.observers {
            observer.update(row, col, e);
        }
    }
}

impl Playground<EnemyWTool, VecDeque<EnemyWTool>> for PlaygroundEnemyImpl {
    fn clear(&mut self) {
        for row in 0..ROW {
            for col in 0..COL {
                self.playground[row][col].clear();
            }
        }
    }

    fn is_empty(&self, row: usize, col: usize) -> bool {
        self.get(row, col).len() == 0
    }

    fn insert(&mut self, row: usize, col: usize, e: &EnemyWTool) -> bool {
        self.playground[row][col].push_back(*e);
        true
    }

    // should solve here using indexes, this way
    // - no need to implement the clone trait fro EnemyWTool
    // - no need to implement the PartialEq trait for EnemyWTool
    // - + performance
    // - less code
    // - here I use remove(index) instead of contains
    // - I can allocate the VecDeque of EnemyWTool
    fn remove(&mut self, value: &VecDeque<EnemyWTool>, row: usize, col: usize) -> bool {
        let mut flag = false;
        for i in 0..FRAMES {
            self.playground[row][col].retain(|e| value.contains(e));
        }
        true
    }

    /*
    fn get(&self, row: usize, col: usize) -> &VecDeque<EnemyWTool> {
        &self.playground[row][col]..&self.playground[row][col + FRAMES]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut VecDeque<EnemyWTool> {
        &mut self.playground[row][col]..&mut self.playground[row][col + FRAMES]
    }
    */

    fn cols(&self) -> usize {
        COL
    }
}
