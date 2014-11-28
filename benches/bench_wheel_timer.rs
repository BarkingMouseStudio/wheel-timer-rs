extern crate test;
extern crate wheel_timer;

use test::Bencher;

use wheel_timer::WheelTimer;

#[bench]
fn bench_wheel_timer_drain(b: &mut Bencher) {
  let max_interval = 20;
  let mut timer = WheelTimer::new(max_interval);

  b.iter(|| {
    // Fill
    for j in range(0u, 100u) {
      timer.schedule(j%max_interval, j%max_interval);
    }

    // Drain
    for _ in timer {
      continue;
    }
  });
}

#[bench]
fn bench_wheel_timer_fill(b: &mut Bencher) {
  let max_interval = 20;
  let mut timer = WheelTimer::new(max_interval);
  let mut i = 0;

  b.iter(|| {
    timer.schedule(i%max_interval, i%max_interval);
    i = i + 1;
  });
}

#[bench]
fn bench_wheel_timer_fast(b: &mut Bencher) {
  let max_interval = 2;
  let mut timer = WheelTimer::new(max_interval);
  let mut i = 0;

  b.iter(|| {
    timer.schedule(i%max_interval, i%max_interval);
    timer.tick();
    i = i + 1;
  });
}
