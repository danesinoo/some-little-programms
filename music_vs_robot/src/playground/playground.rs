use crate::util::observer_playground::ObservablePlayground;

pub trait Playground<T, S>: ObservablePlayground<T> {
    fn clear(&mut self);
    fn is_empty(&self, row: usize, col: usize) -> bool;
    fn insert(&mut self, row: usize, col: usize, value: &T) -> bool;
    fn remove(&mut self, value: &S, row: usize, col: usize) -> bool;
    fn get(&self, row: usize, col: usize) -> &S;
    fn get_mut(&mut self, row: usize, col: usize) -> &mut S;
    fn cols(&self) -> usize;
}
