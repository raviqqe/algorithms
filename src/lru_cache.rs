use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

pub struct LruCache<K, V> {
    // TODO Use a linked list.
    queue: VecDeque<K>,
    map: HashMap<K, V>,
    capacity: usize,
}

impl<K, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Default::default(),
            map: Default::default(),
            capacity,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
            capacity,
        }
    }
}

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.update_key(key);
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.update_key(&key);
        self.map.insert(key, value);
        self.truncate_queue();
    }

    fn update_key(&mut self, key: &K) {
        if let Some(index) = self.queue.iter().position(|other| other == key) {
            self.queue.remove(index);
        }

        self.queue.push_back(key.clone());
    }

    fn truncate_queue(&mut self) {
        if self.queue.len() <= self.capacity {
            return;
        }

        if let Some(key) = self.queue.pop_front() {
            self.map.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let mut cache = LruCache::<_, ()>::new(0);

        assert_eq!(cache.get(&0), None);
    }

    #[test]
    fn insert_key() {
        let mut cache = LruCache::new(1);

        cache.insert(0, 42);

        assert_eq!(cache.get(&0), Some(&42));
    }

    #[test]
    fn insert_keys() {
        let mut cache = LruCache::new(2);

        cache.insert(0, 42);
        cache.insert(1, 2045);

        assert_eq!(cache.get(&0), Some(&42));
        assert_eq!(cache.get(&1), Some(&2045));
    }

    #[test]
    fn push_away_key_out_of_one() {
        let mut cache = LruCache::new(1);

        cache.insert(0, 42);
        cache.insert(1, 2045);

        assert_eq!(cache.get(&0), None);
        assert_eq!(cache.get(&1), Some(&2045));
    }

    #[test]
    fn push_away_key_out_of_two() {
        let mut cache = LruCache::new(1);

        cache.insert(0, 42);
        cache.insert(1, 2023);
        cache.insert(2, 2045);

        assert_eq!(cache.get(&0), None);
        assert_eq!(cache.get(&1), Some(&2045));
    }
}
