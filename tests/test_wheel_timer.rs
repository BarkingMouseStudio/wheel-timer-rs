extern crate wheel_timer;

use wheel_timer::WheelTimer;

#[test]
fn wheel_timer_schedule_test() {
  let mut timer = WheelTimer::new(10);
  timer.schedule(3, "tick");

  timer.tick();
  timer.tick();
  timer.tick();

  let list = timer.tick();
  assert_eq!(list.len(), 1);

  let val = list[0];
  assert_eq!(val, "tick");
}

#[test]
fn wheel_timer_tick_test() {
  let mut timer = WheelTimer::new(10);

  for i in 0..10 {
    timer.schedule(i, i)
  }

  for i in 0..10 {
    let list = timer.tick();
    assert_eq!(list.len(), 1);

    let val = list[0];
    assert_eq!(val, i);
  }
}

#[test]
fn wheel_timer_size_test() {
  let mut timer = Box::new(WheelTimer::new(10));

  for i in 0..10 {
    timer.schedule(i, i)
  }

  assert_eq!(timer.size(), 10);

  for _ in 0..10 {
    timer.tick();
  }

  assert_eq!(timer.size(), 0);
}
