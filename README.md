# clocksource

A Rust library for time and duration types with fixed-size representations,
providing coarse (second-resolution) and precise (nanosecond-resolution)
variants, plus atomic versions for concurrent use. All types use a single
32-bit or 64-bit integer internally, making them compact and efficient for
arithmetic operations.

## Getting Started

```
cargo add clocksource
```

## Usage

```rust
use clocksource::coarse;
use clocksource::precise;

// Coarse (second-resolution) monotonic instant
let coarse_start = coarse::Instant::now();
// ... do work ...
let coarse_elapsed: coarse::Duration = coarse_start.elapsed();
println!("elapsed: {} seconds", coarse_elapsed.as_secs());

// Precise (nanosecond-resolution) monotonic instant
let precise_start = precise::Instant::now();
// ... do work ...
let precise_elapsed: precise::Duration = precise_start.elapsed();
println!("elapsed: {} ns", precise_elapsed.as_nanos());

// Unix timestamps
let now = precise::UnixInstant::now();
let coarse_now = coarse::UnixInstant::now();
```

## Links

- [API documentation (docs.rs)](https://docs.rs/clocksource)
- [Crate page (crates.io)](https://crates.io/crates/clocksource)
- [Repository (GitHub)](https://github.com/iopsystems/clocksource)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your
option.
