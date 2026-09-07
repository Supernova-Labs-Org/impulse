mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use impulse_edge::routing::{index::RouteIndex, scan::scan_lookup};

use support::{ROUTE_SCALES, RouteIndexFixture};

fn benchmark_construction(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("route_index/construction");
    for route_count in ROUTE_SCALES {
        let fixture = RouteIndexFixture::new(route_count);
        group.throughput(Throughput::Elements(route_count as u64));
        group.bench_with_input(
            BenchmarkId::new("from_upstreams", route_count),
            &fixture.upstreams,
            |bencher, upstreams| {
                bencher.iter(|| black_box(RouteIndex::from_upstreams(black_box(upstreams))));
            },
        );
    }
    group.finish();
}

fn benchmark_lookup(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("route_index/lookup");
    for route_count in ROUTE_SCALES {
        let fixture = RouteIndexFixture::new(route_count);

        group.bench_with_input(
            BenchmarkId::new("path_hit", route_count),
            &fixture,
            |bencher, fixture| {
                bencher.iter(|| {
                    black_box(
                        fixture
                            .index
                            .lookup(black_box(fixture.path_hit.as_str()), None),
                    )
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("path_miss", route_count),
            &fixture,
            |bencher, fixture| {
                bencher.iter(|| {
                    black_box(
                        fixture
                            .index
                            .lookup(black_box(fixture.path_miss.as_str()), None),
                    )
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("host_hit", route_count),
            &fixture,
            |bencher, fixture| {
                bencher.iter(|| {
                    black_box(fixture.index.lookup(
                        black_box(fixture.host_hit_path.as_str()),
                        Some(black_box(fixture.host.as_str())),
                    ))
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("host_miss", route_count),
            &fixture,
            |bencher, fixture| {
                bencher.iter(|| {
                    black_box(fixture.index.lookup(
                        black_box(fixture.host_miss_path.as_str()),
                        Some(black_box(fixture.host.as_str())),
                    ))
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("method_hit", route_count),
            &fixture,
            |bencher, fixture| {
                bencher.iter(|| {
                    black_box(fixture.index.lookup_for_method(
                        black_box(fixture.method_path.as_str()),
                        None,
                        Some(black_box(fixture.method.as_str())),
                    ))
                });
            },
        );
    }
    group.finish();
}

fn benchmark_indexed_vs_linear_comparison(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("route_index/comparison");
    for route_count in ROUTE_SCALES {
        let fixture = RouteIndexFixture::new(route_count);

        group.bench_with_input(
            BenchmarkId::new("indexed_path_hit", route_count),
            &fixture,
            |bencher, fixture| {
                bencher.iter(|| {
                    black_box(
                        fixture
                            .index
                            .lookup(black_box(fixture.path_hit.as_str()), None),
                    )
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("linear_path_hit", route_count),
            &fixture,
            |bencher, fixture| {
                bencher.iter(|| {
                    black_box(scan_lookup(
                        black_box(&fixture.upstreams),
                        black_box(fixture.path_hit.as_str()),
                        None,
                    ))
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    route_index_benches,
    benchmark_construction,
    benchmark_lookup,
    benchmark_indexed_vs_linear_comparison
);
criterion_main!(route_index_benches);
