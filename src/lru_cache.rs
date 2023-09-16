pub struct LruCache<K, V> {
    queue: BinaryHeap<(Instant, K)>,
}
