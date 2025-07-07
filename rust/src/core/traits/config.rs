pub trait Config {
    type Error;
    
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, Self::Error>;
    fn set<T: serde::Serialize>(&self, key: &str, value: T) -> Result<(), Self::Error>;
    fn has(&self, key: &str) -> bool;
    fn remove(&self, key: &str) -> Result<(), Self::Error>;
} 