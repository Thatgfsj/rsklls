//! Benchmarks for rsklls framework

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_skill_execution(c: &mut Criterion) {
    c.bench_function("skill_execution_empty", |b| {
        b.iter(|| {
            // Simulate skill execution
            let result = format!("Hello, {}!", "World");
            black_box(result);
        });
    });
}

fn benchmark_registry_operations(c: &mut Criterion) {
    use rsklls::skill::{Registry, SkillInput, SkillTrait};
    use std::collections::HashMap;
    
    struct BenchSkill;
    
    impl SkillTrait for BenchSkill {
        fn execute(&self, _input: &SkillInput) -> rsklls::Result<SkillOutput> {
            Ok(SkillOutput::default())
        }
        fn name(&self) -> &str { "bench_skill" }
        fn description(&self) -> &str { "Benchmark skill" }
    }
    
    c.bench_function("registry_register", |b| {
        b.iter(|| {
            let mut registry = Registry::new();
            let _ = registry.register(BenchSkill);
        });
    });
}

fn benchmark_config_parsing(c: &mut Criterion) {
    use rsklls::Config;
    
    c.bench_function("config_default", |b| {
        b.iter(|| {
            let _config = Config::default();
        });
    });
}

criterion_group!(
    benches,
    benchmark_skill_execution,
    benchmark_registry_operations,
    benchmark_config_parsing
);
criterion_main!(benches);
