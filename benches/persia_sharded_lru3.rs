use bustle::*;
use hashlink::linked_hash_map::RawEntryMut;
use persia_sharded::Sharded;
use std::sync::Arc;

#[derive(Clone)]
struct Table(std::sync::Arc<Sharded<hashlink::LinkedHashMap<u64, Arc<()>>, u64>>);

impl Collection for Table {
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        let mut inner = vec![hashlink::LinkedHashMap::with_capacity(capacity / 128); 128];
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

impl CollectionHandle for Table {
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        let mut entry = self.0.index(key).write();
        let mut entry = entry.raw_entry_mut().from_key(key);
        match entry {
            RawEntryMut::Occupied(ref mut x) => {
                x.to_back();
                true
            }
            RawEntryMut::Vacant(_) => false,
        }
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
        let mut map = self.0.index(key).write();
        if let Some(mut e) = map.get_mut(key) {
            map.insert(*key, Arc::new(()));
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
