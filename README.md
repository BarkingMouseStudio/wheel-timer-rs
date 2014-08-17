Hashed Wheel Timer
===

Simple hashed wheel timer. See http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf

Port of https://github.com/BarkingMouseStudio/wheel_timer to Rust for learning.

Benchmarks
---

Rust implementation:

```
test bench_wheel_timer_drain ... bench:      3935 ns/iter (+/- 548)
test bench_wheel_timer_fast  ... bench:        46 ns/iter (+/- 26)
test bench_wheel_timer_fill  ... bench:        14 ns/iter (+/- 4)
```

Go implementation:

```
BenchmarkWheelTimer_drain   200000       14081 ns/op
BenchmarkWheelTimer_fast   5000000         407 ns/op
BenchmarkWheelTimer_fill  10000000         152 ns/op
```
