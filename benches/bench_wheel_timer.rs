extern crate test;
extern crate wheel_timer;

use test::Bencher;

use wheel_timer::WheelTimer;

#[bench]
fn bench_wheel_timer_drain(b: &mut Bencher) {
  let maxInterval = 20;
  let mut timer = WheelTimer::new(maxInterval);

  b.iter(|| {
    // Fill
    for j in range(0u, 100u) {
      timer.schedule(j%maxInterval, j%maxInterval);
    }

    // Drain
    for k in timer {
      continue;
    }
  });
}

#[bench]
fn bench_wheel_timer_fill(b: &mut Bencher) {
  let maxInterval = 20;
  let mut timer = WheelTimer::new(maxInterval);
  let mut i = 0;

  b.iter(|| {
    timer.schedule(i%maxInterval, i%maxInterval);
    i = i + 1;
  });
}

#[bench]
fn bench_wheel_timer_fast(b: &mut Bencher) {
  let maxInterval = 2;
  let mut timer = WheelTimer::new(maxInterval);
  let mut i = 0;

  b.iter(|| {
    timer.schedule(i%maxInterval, i%maxInterval);
    timer.tick();
    i = i + 1;
  });
}
