use crate::util::observer_playground::ObservablePlayground;

pub trait Playground<T, S>: ObservablePlayground<T> {
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
    fn insert(&mut self, value: &T) -> bool;
    fn remove(&mut self, value: &T, row: usize, col: usize) -> bool;
    fn get(&self, row: usize, col: usize) -> Option<&S>;
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut S>;
}
