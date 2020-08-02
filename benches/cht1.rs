use bustle::*;
use cht1::HashMap;

#[derive(Clone)]
struct Table(std::sync::Arc<HashMap<u64, ()>>);

impl Collection for Table {
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        Self(std::sync::Arc::new(HashMap::with_capacity(capacity)))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl CollectionHandle for Table {
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, ()).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.insert_entry(*key, ()).is_some()
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    println!("embedding server");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(
            n,
            Mix {
                read: 90,
                insert: 10,
                remove: 0,
                update: 0,
                upsert: 0,
            },
        )
        .run::<Table>();
    }
    println!("read heavy");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::read_heavy()).run::<Table>();
    }
    println!("uniform");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::uniform()).run::<Table>();
    }
}
