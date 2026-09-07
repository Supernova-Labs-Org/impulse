# Benchmarking

Impulse uses local Criterion microbenchmarks for focused performance investigation.

## Run Locally

```bash
cargo bench -p impulse-edge --bench route_index
cargo bench -p impulse-lb --bench load_balancing
```

Equivalent Make targets are `bench-route-index` and `bench-load-balancing`.

## Coverage

- route-index construction and indexed lookup behavior
- load-balancer selection and pool/index construction

Benchmarks use real routing and load-balancing APIs with fixtures created outside timed lookup and selection loops.

## CI Policy

Criterion benchmarks are opt-in local tools. CI does not run benchmarks, compare performance baselines, or gate changes on hosted-runner timing.

Use benchmark results to investigate a focused change on comparable local hardware.
