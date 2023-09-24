pub trait Observer {
    fn update(&self, data: &str);
}

pub trait Subject {
    fn register(&mut self, observer: Box<dyn Observer>);
    fn remove(&mut self, observer: Box<dyn Observer>);
    fn notify(&self);
}
