use hibitgraph::BitGraph;
use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};

fn add_edge(c: &mut Criterion) {
    c.bench_function("add_edge", |b| {
        b.iter_batched(
            || BitGraph::with_capacity(1000),
            |mut graph| {
                for u in 0..1000 {
                    for v in (u + 1)..1000 {
                        graph.add_edge(u, v);
                    }
                }
            },
            SmallInput,
        )
    });
}

fn add_edge_unchecked(c: &mut Criterion) {
    c.bench_function("add_edge_unchecked", |b| {
        b.iter_batched(
            || BitGraph::with_capacity(1000),
            |mut graph| {
                for u in 0..1000 {
                    for v in (u + 1)..1000 {
                        graph.add_edge_unchecked(u, v);
                    }
                }
            },
            SmallInput,
        )
    });
}

fn remove_edge(c: &mut Criterion) {
    c.bench_function("remove_edge", |b| {
        b.iter_batched(
            || BitGraph::complete(1000),
            |mut graph| {
                for u in 0..1000 {
                    for v in (u + 1)..1000 {
                        graph.remove_edge(u, v);
                    }
                }
            },
            SmallInput,
        )
    });
}

fn remove_edge_unchecked(c: &mut Criterion) {
    c.bench_function("remove_edge_unchecked", |b| {
        b.iter_batched(
            || BitGraph::complete(1000),
            |mut graph| {
                for u in 0..1000 {
                    for v in (u + 1)..1000 {
                        graph.remove_edge_unchecked(u, v);
                    }
                }
            },
            SmallInput,
        )
    });
}

fn contract(c: &mut Criterion) {
    c.bench_function("contract", |b| {
        b.iter_batched(
            || BitGraph::complete(100),
            |mut graph| {
                for slice in (0..99).zip(1..100) {
                    graph.contract_edge(slice.1, slice.0)
                }
            },
            SmallInput,
        )
    });
}

criterion_group!(bench, contract, add_edge, add_edge_unchecked, remove_edge, remove_edge_unchecked);
criterion_main!(bench);
