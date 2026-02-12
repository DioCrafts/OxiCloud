# 05 - LTO Optimizations

OxiCloud uses Link Time Optimization (LTO) to improve runtime performance. LTO allows the compiler to optimize across module boundaries during linking -- better inlining, dead code elimination, and more efficient binaries.

---

## Implemented Optimizations

### Release Profile
```toml
[profile.release]
lto = "fat"         # Full cross-module optimization
codegen-units = 1   # Maximum optimization but slower compile time
opt-level = 3       # Maximum optimization level
panic = "abort"     # Smaller binary size by removing panic unwinding
strip = true        # Removes debug symbols for smaller binary
```

### Development Profile
```toml
[profile.dev]
opt-level = 1       # Light optimization for faster build time
debug = true        # Keep debug information for development
```

### Benchmark Profile
```toml
[profile.bench]
lto = "fat"         # Full optimization for benchmarks
codegen-units = 1   # Maximum optimization
opt-level = 3       # Maximum optimization level
```

---

## Performance Effects

1. **Smaller binary size** -- unused code and metadata removed
2. **Faster execution** -- better inlining and code optimizations
3. **Reduced memory usage** -- more efficient code layout

---

## LTO Options

- **fat**: Full LTO across all crate boundaries. Maximum optimization, longest compile time.
- **thin**: Faster LTO that trades some optimization for compile speed. Good for development.
- **off**: No cross-module optimization.

---

## Build Time Impact

LTO increases compilation time. The tradeoff:

- Development builds: minimal LTO (`opt-level = 1`) for faster iteration
- Release builds: full LTO for maximum runtime performance
- Benchmark builds: full LTO to measure actual optimized performance

---

## Measuring Impact

```bash
# Run benchmarks with all optimizations
cargo bench

# Compare with non-optimized build (remove for comparison only)
RUSTFLAGS="-C lto=off" cargo bench
```

---

## When to Adjust

Consider changing these settings if:

1. You need faster compile times during development
2. You're experiencing unexpected runtime behavior
3. You want to experiment with optimization vs. binary size tradeoffs

The defaults work well for most cases.
