pub trait ObserverPlayground<T> {
    fn update(&mut self, row: usize, col: usize, e: Option<&T>);
}

pub trait ObservablePlayground<T> {
    fn register(&mut self, observer: Box<dyn ObserverPlayground<T>>);
    fn notify(&mut self, row: usize, col: usize, e: Option<&T>);
}
