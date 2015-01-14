Hashed Wheel Timer
===

Simple hashed wheel timer. See http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf

Port of https://github.com/BarkingMouseStudio/wheel_timer to Rust for learning.

Documentation
---

Impls: `Iterator`

Create a new timer with the specified max interval:

    new(max_interval: uint) -> WheelTimer<T>

Return the number of items currently scheduled:

    size(&self) -> uint

Schedule a new value, available after `ticks`:

    schedule(&mut self, ticks: uint, value: T)

Tick the timer, returning the node at the current tick:

    tick(&mut self) -> Vec<T>


Benchmarks
---

Rust implementation (vec):

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

Rust implementation (list):

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
