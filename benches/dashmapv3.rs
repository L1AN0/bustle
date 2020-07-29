use bustle::*;
use dashmap::DashMap;

#[derive(Clone)]
struct Table<K>(std::sync::Arc<DashMap<K, ()>>);

impl<K> Collection for Table<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        Self(std::sync::Arc::new(DashMap::with_capacity(capacity)))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for Table<K>
where
    K: Send + From<u64> + Copy + 'static + std::hash::Hash + Eq,
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
        use dashmap::mapref::entry::Entry;
        if let Entry::Occupied(mut e) = self.0.entry(*key) {
            e.insert(());
            true
        } else {
            false
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    for n in (1..=num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::uniform()).run::<Table<u64>>();
    }
}
