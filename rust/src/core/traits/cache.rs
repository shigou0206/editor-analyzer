pub trait Cache {
    type Key;
    type Value;
    type Error;
    
    fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>, Self::Error>;
    fn set(&self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error>;
    fn remove(&self, key: &Self::Key) -> Result<(), Self::Error>;
    fn clear(&self) -> Result<(), Self::Error>;

    /// Optional: initialize with capacity
    fn with_capacity(_capacity: usize) -> Result<Self, Self::Error> where Self: Sized {
        Err(unsafe { std::mem::zeroed() })
    }

    /// Optional: evict specific key
    fn evict(&self, _key: &Self::Key) -> Result<(), Self::Error> {
        self.remove(_key)
    }

    /// Current cache element count
    fn len(&self) -> usize {
        0
    }

    /// Cache capacity (if supported)
    fn capacity(&self) -> Option<usize> {
        None
    }
} 