pub struct Cached<K, T> {
    cache: Option<(K, T)>,
}

impl<K: PartialEq, T:> Cached<K, T> {
    pub fn new() -> Self {
        Self { cache: None }
    }

    pub fn get(&mut self, key: K, compute: impl FnOnce(&K) -> T) -> &T {
        if let Some((cached_key, value)) = self.cache.take() && cached_key == key {
            &self.cache.insert((cached_key, value)).1
        } else {
            let value = compute(&key);
            &self.cache.insert((key, value)).1
        }
    }
}
