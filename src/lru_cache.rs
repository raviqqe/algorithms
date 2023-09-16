pub struct LruCache<K, V> {
    queue: BinaryHeap<(Instant, K)>,
    map: HashMap<K, V>,
}

impl<K, V> LruCache<K, V> {
    pub fn new() -> Self {
        Self {
            queue: Default::default(),
            map: Default::default(),
        }
    }

    pub fn get(&mut self, key: K) -> Self {
        self.get()
    }

    fn update_queue(&mut self, key: K) {}
}
