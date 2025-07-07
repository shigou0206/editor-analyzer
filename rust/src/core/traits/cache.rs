pub trait Cache {
    type Key;
    type Value;
    type Error;
    
    fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>, Self::Error>;
    fn set(&self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error>;
    fn set_with_ttl(&self, key: Self::Key, value: Self::Value, ttl: std::time::Duration) -> Result<(), Self::Error>;
    fn remove(&self, key: &Self::Key) -> Result<(), Self::Error>;
    fn clear(&self) -> Result<(), Self::Error>;

    /// Optional: initialize with capacity
    /// 
    /// # Note
    /// 此方法是可选的，默认实现返回错误。
    /// 如果缓存类型支持容量限制，应该覆盖此方法。
    fn with_capacity(_capacity: usize) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: From<&'static str>,
    {
        Err(Self::Error::from("with_capacity not supported for this cache type"))
    }

    /// Optional: evict specific key
    /// 
    /// # Note
    /// 此方法是可选的，默认实现调用 remove()。
    /// 对于 LRU 等缓存策略，应该覆盖此方法以避免统计错误。
    fn evict(&self, key: &Self::Key) -> Result<(), Self::Error> {
        self.remove(key)
    }

    /// Current cache element count
    /// 
    /// # Note
    /// This is a required method that must be implemented by cache providers.
    /// Returns the actual number of elements currently stored in the cache.
    fn len(&self) -> usize;

    /// Cache capacity (if supported)
    /// 
    /// # Note
    /// Returns the maximum number of elements the cache can hold,
    /// or None if the cache has no fixed capacity limit.
    fn capacity(&self) -> Option<usize>;

    /// Check if cache is empty
    /// 
    /// # Note
    /// 默认实现基于 len() 方法，对于大多数缓存类型是合适的。
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Get cache statistics
    /// 
    /// # Note
    /// 返回缓存的统计信息，包括命中率、过期项目数等。
    /// 默认实现返回基本的统计信息。
    fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.len(),
            capacity: self.capacity(),
            hit_rate: 0.0,
            expired_items: 0,
            evicted_items: 0,
        }
    }
    
    /// Clean expired items
    /// 
    /// # Note
    /// 清理过期的缓存项目。
    /// 默认实现不做任何操作。
    fn cleanup(&self) -> Result<usize, Self::Error> {
        Ok(0)
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub capacity: Option<usize>,
    pub hit_rate: f64,
    pub expired_items: usize,
    pub evicted_items: usize,
} 