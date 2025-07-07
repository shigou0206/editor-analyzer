use crate::core::traits::{Cache, ObjectPool, Config};
use crate::core::errors::CoreError;
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::Hash;

/// In-memory cache implementation
pub struct MemoryCache<K, V> {
    storage: DashMap<K, V>,
}

impl<K, V> MemoryCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
        }
    }

    pub fn with_capacity(_capacity: usize) -> Self {
        // DashMap::with_capacity 可用，但这里简单实现
        Self {
            storage: DashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn capacity(&self) -> Option<usize> {
        // DashMap 没有容量概念，返回 None
        None
    }
}

impl<K, V> Cache for MemoryCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    type Key = K;
    type Value = V;
    type Error = CoreError;

    fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>, Self::Error> {
        Ok(self.storage.get(key).map(|v| v.clone()))
    }

    fn set(&self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
        self.storage.insert(key, value);
        Ok(())
    }

    fn remove(&self, key: &Self::Key) -> Result<(), Self::Error> {
        self.storage.remove(key);
        Ok(())
    }

    fn clear(&self) -> Result<(), Self::Error> {
        self.storage.clear();
        Ok(())
    }

    fn with_capacity(capacity: usize) -> Result<Self, Self::Error> {
        Ok(Self::with_capacity(capacity))
    }

    fn evict(&self, key: &Self::Key) -> Result<(), Self::Error> {
        self.remove(key)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> Option<usize> {
        self.capacity()
    }
}

/// Simple object pool implementation
pub struct SimpleObjectPool<T> {
    objects: Arc<Mutex<Vec<T>>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> SimpleObjectPool<T>
where
    T: Send + 'static,
{
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            objects: Arc::new(Mutex::new(Vec::new())),
            factory: Box::new(factory),
        }
    }

    pub fn with_initial_capacity<F>(factory: F, capacity: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let pool = Self::new(factory);
        {
            let mut objects = pool.objects.lock().unwrap();
            for _ in 0..capacity {
                objects.push((pool.factory)());
            }
        }
        pool
    }

    pub fn size(&self) -> usize {
        self.objects.lock().unwrap().len()
    }

    pub fn capacity(&self) -> Option<usize> {
        // Vec 没有硬容量限制，返回 None
        None
    }
}

impl<T> ObjectPool<T> for SimpleObjectPool<T>
where
    T: Send + 'static,
{
    fn acquire(&self) -> Option<T> {
        self.objects.lock().unwrap().pop()
    }

    fn release(&self, item: T) {
        self.objects.lock().unwrap().push(item);
    }

    fn clear(&self) {
        self.objects.lock().unwrap().clear();
    }

    fn size(&self) -> usize {
        self.size()
    }

    fn capacity(&self) -> Option<usize> {
        self.capacity()
    }
}

/// In-memory configuration implementation
pub struct MemoryConfig {
    storage: DashMap<String, serde_json::Value>,
}

impl MemoryConfig {
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<String, serde_json::Value>) -> Self {
        let config = Self::new();
        for (key, value) in map {
            config.storage.insert(key, value);
        }
        config
    }

    /// Validate the value at `key` against a JSON schema string (interface only)
    pub fn validate<T>(&self, _key: &str, _schema: &str) -> Result<bool, CoreError>
    where
        T: serde::de::DeserializeOwned,
    {
        // TODO: Implement JSON Schema validation using schemars or similar
        // For now, always return Ok(true) as a placeholder
        Ok(true)
    }
}

impl Config for MemoryConfig {
    type Error = CoreError;

    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error> {
        self.storage
            .get(key)
            .ok_or_else(|| CoreError::ConfigError {
                code: "config_key_not_found",
                message: format!("Config key '{}' not found", key),
            })
            .and_then(|value| {
                serde_json::from_value(value.clone()).map_err(|e| CoreError::InternalError {
                    code: "config_deserialize_error",
                    message: format!("Failed to deserialize config value: {}", e),
                })
            })
    }

    fn set<T: serde::Serialize>(&self, key: &str, value: T) -> Result<(), Self::Error> {
        let json_value = serde_json::to_value(value).map_err(|e| CoreError::InternalError {
            code: "config_serialize_error",
            message: format!("Failed to serialize config value: {}", e),
        })?;
        self.storage.insert(key.to_string(), json_value);
        Ok(())
    }

    fn has(&self, key: &str) -> bool {
        self.storage.contains_key(key)
    }

    fn remove(&self, key: &str) -> Result<(), Self::Error> {
        self.storage.remove(key);
        Ok(())
    }
}

/// Performance timer utility
pub struct PerformanceTimer {
    start_time: std::time::Instant,
}

impl PerformanceTimer {
    pub fn start() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    pub fn elapsed_millis(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }

    pub fn elapsed_micros(&self) -> u64 {
        self.elapsed().as_micros() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_cache() {
        let cache: MemoryCache<String, i32> = MemoryCache::new();
        
        assert!(cache.set("key1".to_string(), 42).is_ok());
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), Some(42));
        assert_eq!(cache.get(&"key2".to_string()).unwrap(), None);
        
        assert!(cache.remove(&"key1".to_string()).is_ok());
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), None);
    }

    #[test]
    fn test_memory_cache_concurrent() {
        use std::sync::Arc;
        use std::thread;
        
        let cache: Arc<MemoryCache<String, i32>> = Arc::new(MemoryCache::new());
        let mut handles = vec![];
        
        // Spawn multiple threads to test concurrent access
        for i in 0..10 {
            let cache_clone = cache.clone();
            let handle = thread::spawn(move || {
                let key = format!("key{}", i);
                assert!(cache_clone.set(key.clone(), i).is_ok());
                assert_eq!(cache_clone.get(&key).unwrap(), Some(i));
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(cache.len(), 10);
    }

    #[test]
    fn test_memory_cache_clear() {
        let cache: MemoryCache<String, i32> = MemoryCache::new();
        
        assert!(cache.set("key1".to_string(), 42).is_ok());
        assert!(cache.set("key2".to_string(), 100).is_ok());
        assert_eq!(cache.len(), 2);
        
        assert!(cache.clear().is_ok());
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_object_pool() {
        let pool = SimpleObjectPool::new(|| String::new());
        
        // Initially pool is empty, so acquire should return None
        assert!(pool.acquire().is_none());
        
        // Release some items
        pool.release("item1".to_string());
        pool.release("item2".to_string());
        
        // Now should be able to acquire
        let item1 = pool.acquire().unwrap();
        let item2 = pool.acquire().unwrap();
        
        // LIFO order, so last released should be first acquired
        assert_eq!(item1, "item2"); // First acquired is last released
        assert_eq!(item2, "item1"); // Second acquired is first released
        
        // Pool should be empty again
        assert!(pool.acquire().is_none());
    }

    #[test]
    fn test_object_pool_with_initial_capacity() {
        let pool = SimpleObjectPool::with_initial_capacity(|| String::from("default"), 3);
        
        // Should have 3 items initially
        assert!(pool.acquire().is_some());
        assert!(pool.acquire().is_some());
        assert!(pool.acquire().is_some());
        
        // Pool should be empty now
        assert!(pool.acquire().is_none());
        
        // Release items back
        pool.release("item1".to_string());
        pool.release("item2".to_string());
        
        // Should be able to acquire again
        assert!(pool.acquire().is_some());
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn test_object_pool_clear() {
        let pool = SimpleObjectPool::new(|| String::new());
        
        pool.release("item1".to_string());
        pool.release("item2".to_string());
        
        assert_eq!(pool.acquire().unwrap(), "item2"); // LIFO order
        assert_eq!(pool.acquire().unwrap(), "item1");
        
        pool.release("item3".to_string());
        pool.clear();
        
        // Pool should be empty after clear
        assert!(pool.acquire().is_none());
    }

    #[test]
    fn test_memory_config() {
        let config = MemoryConfig::new();
        
        assert!(config.set("test_key", 42).is_ok());
        assert_eq!(config.get::<i32>("test_key").unwrap(), 42);
        assert!(config.has("test_key"));
        
        assert!(config.remove("test_key").is_ok());
        assert!(!config.has("test_key"));
    }

    #[test]
    fn test_memory_config_from_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), serde_json::json!(42));
        map.insert("key2".to_string(), serde_json::json!("value"));
        
        let config = MemoryConfig::from_map(map);
        
        assert_eq!(config.get::<i32>("key1").unwrap(), 42);
        assert_eq!(config.get::<String>("key2").unwrap(), "value");
    }

    #[test]
    fn test_memory_config_complex_types() {
        let config = MemoryConfig::new();
        
        // Test with complex types
        let vec_data = vec![1, 2, 3, 4, 5];
        assert!(config.set("vector", &vec_data).is_ok());
        assert_eq!(config.get::<Vec<i32>>("vector").unwrap(), vec_data);
        
        let struct_data = serde_json::json!({
            "name": "test",
            "value": 42,
            "nested": {
                "key": "value"
            }
        });
        assert!(config.set("struct", &struct_data).is_ok());
        let retrieved: serde_json::Value = config.get("struct").unwrap();
        assert_eq!(retrieved, struct_data);
    }

    #[test]
    fn test_memory_config_error_handling() {
        let config = MemoryConfig::new();
        
        // Test getting non-existent key
        let result: Result<i32, _> = config.get("non_existent");
        assert!(result.is_err());
        
        // Test type mismatch
        assert!(config.set("string_key", "hello").is_ok());
        let result: Result<i32, _> = config.get("string_key");
        assert!(result.is_err());
    }

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::start();
        
        // Do some work
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);
        
        let elapsed_millis = timer.elapsed_millis();
        assert!(elapsed_millis >= 10);
        
        let elapsed_micros = timer.elapsed_micros();
        assert!(elapsed_micros >= 10000); // 10ms = 10000μs
    }

    #[test]
    fn test_memory_cache_different_types() {
        // Test with different key and value types
        let string_cache: MemoryCache<String, String> = MemoryCache::new();
        assert!(string_cache.set("key".to_string(), "value".to_string()).is_ok());
        assert_eq!(string_cache.get(&"key".to_string()).unwrap(), Some("value".to_string()));
        
        let int_cache: MemoryCache<i32, bool> = MemoryCache::new();
        assert!(int_cache.set(42, true).is_ok());
        assert_eq!(int_cache.get(&42).unwrap(), Some(true));
    }

    #[test]
    fn test_object_pool_concurrent() {
        use std::sync::Arc;
        use std::thread;
        
        let pool: Arc<SimpleObjectPool<String>> = Arc::new(SimpleObjectPool::new(|| String::new()));
        let mut handles = vec![];
        
        // Spawn multiple threads to test concurrent pool access
        for _ in 0..5 {
            let pool_clone = pool.clone();
            let handle = thread::spawn(move || {
                let item = pool_clone.acquire().unwrap_or_else(|| "default".to_string());
                pool_clone.release(item);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_memory_cache_with_capacity_and_capacity() {
        let cache = MemoryCache::<String, i32>::with_capacity(128);
        assert_eq!(cache.len(), 0);
        // capacity() returns None for DashMap-based cache
        assert_eq!(cache.capacity(), None);
    }

    #[test]
    fn test_memory_cache_evict() {
        let cache: MemoryCache<String, i32> = MemoryCache::new();
        cache.set("evict_key".to_string(), 99).unwrap();
        assert_eq!(cache.get(&"evict_key".to_string()).unwrap(), Some(99));
        cache.evict(&"evict_key".to_string()).unwrap();
        assert_eq!(cache.get(&"evict_key".to_string()).unwrap(), None);
    }

    #[test]
    fn test_object_pool_size_and_capacity() {
        let pool = SimpleObjectPool::with_initial_capacity(|| 42, 5);
        assert_eq!(pool.size(), 5);
        assert_eq!(pool.capacity(), None); // Vec has no hard cap
        for _ in 0..5 { pool.acquire(); }
        assert_eq!(pool.size(), 0);
    }

    #[test]
    fn test_memory_config_validate() {
        let config = MemoryConfig::new();
        config.set("valid", 123).unwrap();
        // Always returns Ok(true) for now
        let valid = config.validate::<i32>("valid", r#"{"type":"integer"}"#);
        assert_eq!(valid, Ok(true));
    }
} 