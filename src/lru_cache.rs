pub struct LruCache<K, V> {
    queue: BinaryHeap<(Instant, K)>,
}

impl<K, V> LruCache<K, V> {
    pub fn new() -> Self {}

    pub fn get(&mut self, key: K) -> Self {}
}
