extern crate test;
extern crate wheel_timer;

use test::Bencher;

use wheel_timer::WheelTimer;
use wheel_timer::Cons;
use wheel_timer::Nil;

#[test]
fn wheel_timer_schedule_test() {
  let mut timer = WheelTimer::new(10);
  timer.schedule(3, "tick");

  timer.tick();
  timer.tick();
  timer.tick();

  let node = timer.tick();
  assert!(node != Nil);

  let val = match node {
    Cons(val, _) => val,
    Nil => ""
  };
  assert!(val == "tick");
}

#[test]
fn wheel_timer_tick_test() {
  let mut timer = WheelTimer::new(10);

  for i in range(0, 10) {
    timer.schedule(i, i)
  }

  for i in range(0, 10) {
    let node = timer.tick();
    assert!(node != Nil);

    let val = match node {
      Cons(val, _) => val,
      Nil => -1
    };
    assert!(val == i);
  }
}

#[test]
fn wheel_timer_size_test() {
  let mut timer = WheelTimer::new(10);

  for i in range(0, 10) {
    timer.schedule(i, i)
  }

  assert!(timer.size() == 10);

  for _ in timer {
  }

  assert!(timer.size() == 0);
}

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
