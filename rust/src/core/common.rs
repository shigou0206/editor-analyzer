use crate::core::traits::{Cache, ObjectPool, Config};
use crate::core::errors::CoreError;
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use std::sync::OnceLock;

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

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: DashMap::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn capacity(&self) -> Option<usize> {
        None
    }
}

impl<K, V> Default for MemoryCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
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

    fn set_with_ttl(&self, key: Self::Key, value: Self::Value, _ttl: std::time::Duration) -> Result<(), Self::Error> {
        // MemoryCache doesn't support TTL, so we just set the value
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
        Ok(MemoryCache::with_capacity(capacity))
    }

    fn evict(&self, key: &Self::Key) -> Result<(), Self::Error> {
        self.remove(key)
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn capacity(&self) -> Option<usize> {
        None
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
        None
    }
}

impl<T> ObjectPool<T> for SimpleObjectPool<T>
where
    T: Send + 'static,
{
    type Error = CoreError;

    fn acquire(&self) -> Result<T, Self::Error> {
        Ok(self.objects.lock().unwrap().pop().unwrap_or_else(|| (self.factory)()))
    }

    fn release(&self, item: T) -> Result<(), Self::Error> {
        self.objects.lock().unwrap().push(item);
        Ok(())
    }

    fn available_count(&self) -> usize {
        self.objects.lock().unwrap().len()
    }

    fn capacity(&self) -> usize {
        0 // No fixed capacity
    }

    fn clear(&self) -> Result<(), Self::Error> {
        self.objects.lock().unwrap().clear();
        Ok(())
    }

    fn stats(&self) -> crate::core::traits::object_pool::PoolStats {
        let len = self.objects.lock().unwrap().len();
        crate::core::traits::object_pool::PoolStats {
            capacity: 0,
            available: len,
            in_use: 0,
            total_created: len,
            total_destroyed: 0,
        }
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
    /// 
    /// # Note
    /// This is currently a stub implementation that returns an error.
    /// For production use, consider implementing proper JSON Schema validation
    /// using libraries like `schemars` or `jsonschema`.
    /// 
    /// # Returns
    /// Returns an error indicating that validation is not implemented.
    /// 
    /// # Future Implementation
    /// To implement this properly:
    /// 1. Add schemars or jsonschema dependency
    /// 2. Implement actual JSON Schema validation
    /// 3. Consider caching compiled schemas for performance
    pub fn validate<T>(&self, _key: &str, _schema: &str) -> Result<bool, CoreError>
    where
        T: serde::de::DeserializeOwned,
    {
        Err(CoreError::InternalError {
            code: "validation_not_implemented",
            message: "MemoryConfig::validate is a stub implementation. Use proper JSON Schema validation libraries like 'schemars' or 'jsonschema' for production.".to_string(),
        })
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self::new()
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

    fn keys(&self) -> Vec<String> {
        self.storage.iter().map(|entry| entry.key().clone()).collect()
    }

    fn get_raw(&self, key: &str) -> Option<serde_json::Value> {
        self.storage.get(key).map(|entry| entry.value().clone())
    }

    fn set_raw(&self, key: &str, value: serde_json::Value) -> Result<(), Self::Error> {
        self.storage.insert(key.to_string(), value);
        Ok(())
    }

    fn load_from_file(&self, _path: &std::path::PathBuf) -> Result<(), Self::Error> {
        Err(CoreError::ConfigError {
            code: "not_implemented",
            message: "load_from_file not implemented for MemoryConfig".to_string(),
        })
    }

    fn save_to_file(&self, _path: &std::path::PathBuf) -> Result<(), Self::Error> {
        Err(CoreError::ConfigError {
            code: "not_implemented",
            message: "save_to_file not implemented for MemoryConfig".to_string(),
        })
    }

    fn load_from_env(&self, _prefix: &str) -> Result<(), Self::Error> {
        Err(CoreError::ConfigError {
            code: "not_implemented",
            message: "load_from_env not implemented for MemoryConfig".to_string(),
        })
    }

    fn validate(&self, _schema: &crate::core::traits::config::ConfigSchema) -> Result<(), crate::core::traits::config::ConfigValidationError> {
        Err(crate::core::traits::config::ConfigValidationError::SchemaError {
            message: "validation not implemented for MemoryConfig".to_string(),
        })
    }

    fn schema(&self) -> Option<&crate::core::traits::config::ConfigSchema> {
        None
    }

    fn set_schema(&self, _schema: crate::core::traits::config::ConfigSchema) -> Result<(), Self::Error> {
        Err(CoreError::ConfigError {
            code: "not_implemented",
            message: "set_schema not implemented for MemoryConfig".to_string(),
        })
    }

    fn reset_to_defaults(&self) -> Result<(), Self::Error> {
        self.storage.clear();
        Ok(())
    }

    fn stats(&self) -> crate::core::traits::config::ConfigStats {
        crate::core::traits::config::ConfigStats {
            total_keys: self.storage.len(),
            loaded_files: Vec::new(),
            last_modified: None,
            validation_errors: Vec::new(),
        }
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

/// Performance metrics collector
#[derive(Debug, Clone)]
pub struct Metrics {
    pub operation_times: HashMap<String, Vec<u64>>,
    pub memory_usage: Vec<usize>,
    pub error_count: usize,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            operation_times: HashMap::new(),
            memory_usage: Vec::new(),
            error_count: 0,
        }
    }

    pub fn record_operation(&mut self, name: &str, duration: std::time::Duration) {
        let millis = duration.as_millis() as u64;
        self.operation_times
            .entry(name.to_string())
            .or_default()
            .push(millis);
    }

    pub fn record_memory_usage(&mut self, bytes: usize) {
        self.memory_usage.push(bytes);
    }

    pub fn record_error(&mut self, _error: &dyn std::error::Error) {
        self.error_count += 1;
    }

    pub fn get_average_time(&self, operation: &str) -> Option<f64> {
        self.operation_times
            .get(operation)
            .map(|times| times.iter().sum::<u64>() as f64 / times.len() as f64)
    }

    pub fn get_max_time(&self, operation: &str) -> Option<u64> {
        self.operation_times
            .get(operation)
            .and_then(|times| times.iter().max().copied())
    }

    pub fn get_memory_stats(&self) -> Option<(usize, usize, f64)> {
        if self.memory_usage.is_empty() {
            return None;
        }
        
        let min = *self.memory_usage.iter().min().unwrap();
        let max = *self.memory_usage.iter().max().unwrap();
        let avg = self.memory_usage.iter().sum::<usize>() as f64 / self.memory_usage.len() as f64;
        
        Some((min, max, avg))
    }

    pub fn clear(&mut self) {
        self.operation_times.clear();
        self.memory_usage.clear();
        self.error_count = 0;
    }

    pub fn record_operation_time(&mut self, operation: String, duration: std::time::Duration) {
        let millis = duration.as_millis() as u64;
        self.operation_times
            .entry(operation)
            .or_default()
            .push(millis);
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Global metrics instance
/// 
/// # Performance Note
/// This uses std::sync::Mutex which may cause contention under high write load.
/// For high-concurrency scenarios, consider using parking_lot::Mutex or sharded metrics.
static METRICS: OnceLock<Mutex<Metrics>> = OnceLock::new();

/// Get global metrics instance
pub fn get_metrics() -> &'static Mutex<Metrics> {
    METRICS.get_or_init(|| Mutex::new(Metrics::new()))
}

/// Record operation time
pub fn record_operation(name: &str, duration: std::time::Duration) {
    if let Ok(mut metrics) = get_metrics().lock() {
        metrics.record_operation(name, duration);
    }
}

/// Record memory usage
pub fn record_memory_usage(bytes: usize) {
    if let Ok(mut metrics) = get_metrics().lock() {
        metrics.record_memory_usage(bytes);
    }
}

/// Record error
pub fn record_error(error: &dyn std::error::Error) {
    if let Ok(mut metrics) = get_metrics().lock() {
        metrics.record_error(error);
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
        
        // Initially pool is empty, but acquire will create new items
        let item = pool.acquire().unwrap();
        assert_eq!(item, "");
        
        // Release some items
        pool.release("item1".to_string()).unwrap();
        pool.release("item2".to_string()).unwrap();
        
        // Now should be able to acquire
        let item1 = pool.acquire().unwrap();
        let item2 = pool.acquire().unwrap();
        
        // LIFO order, so last released should be first acquired
        assert_eq!(item1, "item2"); // First acquired is last released
        assert_eq!(item2, "item1"); // Second acquired is first released
        
        // Pool should be empty again, but acquire will create new items
        let item3 = pool.acquire().unwrap();
        assert_eq!(item3, "");
    }

    #[test]
    fn test_object_pool_with_initial_capacity() {
        let pool = SimpleObjectPool::with_initial_capacity(|| String::from("default"), 3);
        
        // Should have 3 items initially
        assert!(pool.acquire().is_ok());
        assert!(pool.acquire().is_ok());
        assert!(pool.acquire().is_ok());
        
        // Pool should be empty now, but acquire will create new items
        assert!(pool.acquire().is_ok());
        
        // Release items back
        pool.release("item1".to_string()).unwrap();
        pool.release("item2".to_string()).unwrap();
        
        // Should be able to acquire again
        assert!(pool.acquire().is_ok());
        assert!(pool.acquire().is_ok());
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
        
        // Pool should be empty after clear, but acquire will create new items
        assert!(pool.acquire().is_ok());
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
                let item = pool_clone.acquire().unwrap_or_else(|_| "default".to_string());
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
        
        // validate() now returns an error indicating it's not implemented
        let result = config.validate::<i32>("valid", r#"{"type":"integer"}"#);
        assert!(result.is_err());
        
        if let Err(CoreError::InternalError { code, message }) = result {
            assert_eq!(code, "validation_not_implemented");
            assert!(message.contains("stub implementation"));
        } else {
            panic!("Expected InternalError with validation_not_implemented code");
        }
    }
} 