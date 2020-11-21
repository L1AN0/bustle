use bustle::*;
use persia_sharded::Sharded;
use std::sync::Arc;

#[derive(Clone)]
struct Table<K>(std::sync::Arc<Sharded<griddle::HashMap<K, Arc<()>>, K>>);

impl<K> Collection for Table<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        let mut inner = vec![griddle::HashMap::with_capacity(capacity / 128); 128];
        Self(std::sync::Arc::new(Sharded {
            inner: inner
                .into_iter()
                .map(|x| parking_lot::RwLock::new(x))
                .collect(),
            phantom: std::marker::PhantomData::default(),
        }))
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
        self.0.index(key).read().get(key).cloned().is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        !self
            .0
            .index(key)
            .write()
            .insert(*key, Arc::new(()))
            .is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.index(key).write().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        use griddle::hash_map::Entry;
        let mut map = self.0.index(key).write();
        if let Entry::Occupied(mut e) = map.entry(*key) {
            e.insert(Arc::new(()));
            true
        } else {
            false
        }
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
