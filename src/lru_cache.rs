pub struct LruCache<K, V> {
    queue: VecDeque<K>,
    map: HashMap<K, V>,
}

impl<K, V> LruCache<K, V> {
    pub fn new() -> Self {
        Self {
            queue: Default::default(),
            map: Default::default(),
        }
    }

    pub fn get(&mut self, key: K) -> Option<K> {
        self.map.get(key)
    }

    fn update_queue(&mut self, key: K) {}
}
