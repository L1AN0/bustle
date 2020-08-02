use bustle::*;
use contrie::CloneConMap;

#[derive(Clone)]
struct Table<K: Clone + std::hash::Hash + std::cmp::Eq + 'static>(
    std::sync::Arc<CloneConMap<K, ()>>,
);

impl<K> Collection for Table<K>
where
    K: Send
        + Sync
        + From<u64>
        + Copy
        + 'static
        + std::hash::Hash
        + Eq
        + std::fmt::Debug
        + std::cmp::Ord,
{
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        Self(std::sync::Arc::new(CloneConMap::new()))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for Table<K>
where
    K: Send
        + Sync
        + From<u64>
        + Copy
        + 'static
        + std::hash::Hash
        + Eq
        + std::fmt::Debug
        + std::cmp::Ord,
{
    type Key = K;

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
        if self.0.get(key).is_some() {
            self.0.insert(*key, ()).is_some()
        } else {
            false
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    println!("read heavy");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::read_heavy()).run::<Table<u64>>();
    }
    println!("uniform");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::uniform()).run::<Table<u64>>();
    }
}
