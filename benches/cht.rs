use ahash::RandomState;
use bustle::*;
use cht::HashMap;
use fnv::FnvBuildHasher;

#[derive(Clone)]
struct Table<K>(std::sync::Arc<HashMap<K, (), RandomState>>);

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
        Self(std::sync::Arc::new(HashMap::with_capacity_and_hasher(
            capacity,
            RandomState::default(),
        )))
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
        self.0.modify_entry(*key, |_k, _v| ()).is_some()
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
