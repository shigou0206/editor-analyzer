pub trait ObjectPool<T> {
    fn acquire(&self) -> Option<T>;
    fn release(&self, item: T);
    fn clear(&self);

    /// 当前池中可用对象数量
    fn size(&self) -> usize {
        0
    }

    /// 池容量（如支持）
    fn capacity(&self) -> Option<usize> {
        None
    }
} 