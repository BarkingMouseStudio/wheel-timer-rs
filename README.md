Hashed Wheel Timer
===

A simple hashed wheel timer. See http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf

Ported from Go version (https://github.com/BarkingMouseStudio/wheel_timer) to Rust.

Documentation
---

http://barkingmousestudio.com/wheel-timer-rs/wheel_timer/

Benchmarks
---

Rust implementation (using vec):

Pre-alpha:

```
test bench_wheel_timer_drain ... bench:      4644 ns/iter (+/- 628)
test bench_wheel_timer_fast  ... bench:        61 ns/iter (+/- 4)
test bench_wheel_timer_fill  ... bench:        30 ns/iter (+/- 5)
```

Alpha:

```
test bench_wheel_timer_drain ... bench:      3410 ns/iter (+/- 1198)
test bench_wheel_timer_fast  ... bench:        43 ns/iter (+/- 27)
test bench_wheel_timer_fill  ... bench:        20 ns/iter (+/- 3)
```

Rust implementation (using linked list):

```
test bench_wheel_timer_drain ... bench:      8839 ns/iter (+/- 4160)
test bench_wheel_timer_fast  ... bench:       111 ns/iter (+/- 50)
test bench_wheel_timer_fill  ... bench:        73 ns/iter (+/- 33)
```

Go implementation:

```
BenchmarkWheelTimer_drain   200000       14081 ns/op
BenchmarkWheelTimer_fast   5000000         407 ns/op
BenchmarkWheelTimer_fill  10000000         152 ns/op
```
