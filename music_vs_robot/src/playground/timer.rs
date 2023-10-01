use crate::playground::cash::Counter;
use crate::util::observer::{Observer, Subject};

struct Timer {
    obs: Vec<Box<dyn Observer>>,
    count: usize,
}

impl Timer {
    fn new() -> Self {
        Self {
            obs: Vec::new(),
            count: 0,
        }
    }
}

impl Counter for Timer {
    fn get(&self) -> usize {
        self.count
    }

    fn clean(&mut self) {
        self.count = 0;
    }

    fn add(&mut self, n: usize) {
        self.count += n;
    }

    fn sub(&mut self, n: usize) {
        if n < self.count {
            self.count -= n;
        }
    }
}

impl Subject for Timer {
    fn register(&mut self, obs: Box<dyn Observer>) {
        self.obs.push(obs);
    }

    fn remove(&mut self, obs: Box<dyn Observer>) {
        self.obs.retain(|x| !std::ptr::eq(x, &obs));
    }

    fn notify(&self) {
        for obs in &self.obs {
            obs.update(&self.count.to_string());
        }
    }
}
