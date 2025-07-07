use criterion::{criterion_group, criterion_main, Criterion};
use rust::core::common::MemoryCache;
use rust::traits::Cache;

fn bench_memory_cache_set_get(c: &mut Criterion) {
    c.bench_function("memory_cache_set_get", |b| {
        b.iter(|| {
            let cache: MemoryCache<String, i32> = MemoryCache::new();
            for i in 0..1000 {
                let key = format!("key{}", i);
                cache.set(key.clone(), i).unwrap();
                let _ = cache.get(&key).unwrap();
            }
        })
    });
}

criterion_group!(benches, bench_memory_cache_set_get);
criterion_main!(benches); 