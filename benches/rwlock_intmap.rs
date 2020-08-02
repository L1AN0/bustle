use bustle::*;
use intmap::IntMap;
use std::sync::RwLock;

#[derive(Clone)]
struct Table(std::sync::Arc<RwLock<IntMap<()>>>);

impl Collection for Table {
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        let map = IntMap::with_capacity(capacity);
        Self(std::sync::Arc::new(RwLock::new(map)))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl CollectionHandle for Table {
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().unwrap().get(*key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().insert(*key, ())
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().remove(*key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let c = { self.0.read().unwrap().contains_key(*key) };
        if c {
            self.0.write().unwrap().insert(*key, ());
            true
        } else {
            false
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    println!("read heavy");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::read_heavy()).run::<Table>();
    }
    println!("uniform");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::uniform()).run::<Table>();
    }
}
