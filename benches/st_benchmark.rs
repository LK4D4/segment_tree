use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use segment_tree::SegmentTree;

pub fn modify(c: &mut Criterion) {
    let data: Vec<i32> = (1..1000).collect();
    c.bench_function("replace", move |b| {
        b.iter_batched(
            || SegmentTree::new(data.clone()),
            |mut st| st.replace(1, 3),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, modify);
criterion_main!(benches);
