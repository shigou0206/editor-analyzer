use std::sync::{Arc, Mutex, RwLock};
use std::collections::VecDeque;

/// 对象池 trait - 泛型封装
pub trait ObjectPool<T>: Send + Sync {
    type Error;
    
    /// 获取对象
    fn acquire(&self) -> Result<T, Self::Error>;
    
    /// 释放对象
    fn release(&self, obj: T) -> Result<(), Self::Error>;
    
    /// 当前池中可用对象数量
    fn available_count(&self) -> usize;
    
    /// 池的总容量
    fn capacity(&self) -> usize;
    
    /// 检查池是否为空
    fn is_empty(&self) -> bool {
        self.available_count() == 0
    }
    
    /// 检查池是否已满
    fn is_full(&self) -> bool {
        self.available_count() >= self.capacity()
    }
    
    /// 清理池
    fn clear(&self) -> Result<(), Self::Error>;
    
    /// 获取池的统计信息
    fn stats(&self) -> PoolStats;
}

/// 对象池统计信息
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub capacity: usize,
    pub available: usize,
    pub in_use: usize,
    pub total_created: usize,
    pub total_destroyed: usize,
}

impl Default for PoolStats {
    fn default() -> Self {
        Self {
            capacity: 0,
            available: 0,
            in_use: 0,
            total_created: 0,
            total_destroyed: 0,
        }
    }
}

/// 对象工厂 trait
pub trait ObjectFactory<T>: Send + Sync {
    type Error;
    
    /// 创建新对象
    fn create(&self) -> Result<T, Self::Error>;
    
    /// 重置对象状态
    fn reset(&self, obj: &mut T) -> Result<(), Self::Error>;
    
    /// 验证对象是否有效
    fn validate(&self, obj: &T) -> bool;
    
    /// 销毁对象
    fn destroy(&self, obj: T) -> Result<(), Self::Error>;
}

/// 线程安全的对象池实现
pub struct ThreadSafePool<T, F> {
    objects: Arc<RwLock<VecDeque<T>>>,
    factory: Arc<F>,
    capacity: usize,
    stats: Arc<Mutex<PoolStats>>,
}

impl<T, F> ThreadSafePool<T, F>
where
    T: Send + Sync + 'static,
    F: ObjectFactory<T>,
{
    pub fn new(factory: F, capacity: usize) -> Self {
        let stats = PoolStats {
            capacity,
            available: 0,
            in_use: 0,
            total_created: 0,
            total_destroyed: 0,
        };
        
        Self {
            objects: Arc::new(RwLock::new(VecDeque::new())),
            factory: Arc::new(factory),
            capacity,
            stats: Arc::new(Mutex::new(stats)),
        }
    }
    
    fn update_stats(&self, f: impl FnOnce(&mut PoolStats)) {
        if let Ok(mut stats) = self.stats.lock() {
            f(&mut stats);
        }
    }
}

impl<T, F> ObjectPool<T> for ThreadSafePool<T, F>
where
    T: Send + Sync + 'static,
    F: ObjectFactory<T>,
{
    type Error = F::Error;
    
    fn acquire(&self) -> Result<T, Self::Error> {
        // 尝试从池中获取对象
        if let Ok(mut objects) = self.objects.write() {
            if let Some(mut obj) = objects.pop_front() {
                // 重置对象状态
                self.factory.reset(&mut obj)?;
                
                self.update_stats(|stats| {
                    stats.available -= 1;
                    stats.in_use += 1;
                });
                
                return Ok(obj);
            }
        }
        
        // 池为空，创建新对象
        let obj = self.factory.create()?;
        
        self.update_stats(|stats| {
            stats.total_created += 1;
            stats.in_use += 1;
        });
        
        Ok(obj)
    }
    
    fn release(&self, obj: T) -> Result<(), Self::Error> {
        // 验证对象
        if !self.factory.validate(&obj) {
            self.factory.destroy(obj)?;
            
            self.update_stats(|stats| {
                stats.total_destroyed += 1;
                stats.in_use -= 1;
            });
            
            return Ok(());
        }
        
        // 检查池是否已满
        if let Ok(mut objects) = self.objects.write() {
            if objects.len() < self.capacity {
                objects.push_back(obj);
                
                self.update_stats(|stats| {
                    stats.available += 1;
                    stats.in_use -= 1;
                });
                
                return Ok(());
            }
        }
        
        // 池已满，销毁对象
        self.factory.destroy(obj)?;
        
        self.update_stats(|stats| {
            stats.total_destroyed += 1;
            stats.in_use -= 1;
        });
        
        Ok(())
    }
    
    fn available_count(&self) -> usize {
        self.objects.read().map(|objects| objects.len()).unwrap_or(0)
    }
    
    fn capacity(&self) -> usize {
        self.capacity
    }
    
    fn clear(&self) -> Result<(), Self::Error> {
        if let Ok(mut objects) = self.objects.write() {
            let count = objects.len();
            for obj in objects.drain(..) {
                self.factory.destroy(obj)?;
            }
            
            self.update_stats(|stats| {
                stats.available = 0;
                stats.total_destroyed += count;
            });
        }
        
        Ok(())
    }
    
    fn stats(&self) -> PoolStats {
        self.stats.lock().map(|stats| stats.clone()).unwrap_or_default()
    }
}

/// 简单的对象工厂实现
pub struct SimpleFactory<T, CreateFn, ResetFn, ValidateFn, DestroyFn>
where
    CreateFn: Fn() -> Result<T, String> + Send + Sync,
    ResetFn: Fn(&mut T) -> Result<(), String> + Send + Sync,
    ValidateFn: Fn(&T) -> bool + Send + Sync,
    DestroyFn: Fn(T) -> Result<(), String> + Send + Sync,
{
    create: CreateFn,
    reset: ResetFn,
    validate: ValidateFn,
    destroy: DestroyFn,
}

impl<T, CreateFn, ResetFn, ValidateFn, DestroyFn> SimpleFactory<T, CreateFn, ResetFn, ValidateFn, DestroyFn>
where
    CreateFn: Fn() -> Result<T, String> + Send + Sync,
    ResetFn: Fn(&mut T) -> Result<(), String> + Send + Sync,
    ValidateFn: Fn(&T) -> bool + Send + Sync,
    DestroyFn: Fn(T) -> Result<(), String> + Send + Sync,
{
    pub fn new(create: CreateFn, reset: ResetFn, validate: ValidateFn, destroy: DestroyFn) -> Self {
        Self {
            create,
            reset,
            validate,
            destroy,
        }
    }
}

impl<T, CreateFn, ResetFn, ValidateFn, DestroyFn> ObjectFactory<T> for SimpleFactory<T, CreateFn, ResetFn, ValidateFn, DestroyFn>
where
    CreateFn: Fn() -> Result<T, String> + Send + Sync,
    ResetFn: Fn(&mut T) -> Result<(), String> + Send + Sync,
    ValidateFn: Fn(&T) -> bool + Send + Sync,
    DestroyFn: Fn(T) -> Result<(), String> + Send + Sync,
{
    type Error = String;
    
    fn create(&self) -> Result<T, Self::Error> {
        (self.create)()
    }
    
    fn reset(&self, obj: &mut T) -> Result<(), Self::Error> {
        (self.reset)(obj)
    }
    
    fn validate(&self, obj: &T) -> bool {
        (self.validate)(obj)
    }
    
    fn destroy(&self, obj: T) -> Result<(), Self::Error> {
        (self.destroy)(obj)
    }
} 