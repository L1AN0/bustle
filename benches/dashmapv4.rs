use bustle::*;
use dashmapv4::DashMap;

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
    K: Send + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        !self.0.insert(*key, ())
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key)
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.update(key, |_k, _v| ())
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
        .run::<Table<u64>>();
    }
    println!("read heavy");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::read_heavy()).run::<Table<u64>>();
    }
    println!("uniform");
    for n in (1..=2 * num_cpus::get()).step_by(num_cpus::get() / 4) {
        Workload::new(n, Mix::uniform()).run::<Table<u64>>();
    }
}
