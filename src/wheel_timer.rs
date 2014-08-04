extern crate test;

use test::Bencher;

use std::mem;
use std::fmt::{Show, Formatter, Result};
use std::iter::AdditiveIterator;

enum Node<T> {
  Cons(T, Box<Node<T>>),
  Nil
}

impl<T> Node<T> {
  fn prepend(self, elem: T) -> Node<T> {
    Cons(elem, box self)
  }

  fn len(&self) -> uint {
    match *self {
      Cons(_, ref tail) => tail.len() + 1,
      Nil => 0
    }
  }
}

impl<T: Show> Show for Node<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Cons(ref head, ref tail) => {
        write!(f, "{}, [ ] -> {}", head, tail)
      },
      Nil => {
        write!(f, "Nil")
      },
    }
  }
}

impl<T: PartialEq> PartialEq for Node<T> {
  fn eq(&self, ys: &Node<T>) -> bool {
    match (self, ys) {
      (&Nil, &Nil) => true,
      (&Cons(ref x, box ref next_xs), &Cons(ref y, box ref next_ys))
        if x == y => next_xs == next_ys,
      _ => false
    }
  }
}

// Simple hashed wheel timer with bounded interval
// See http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf
pub struct WheelTimer<T> {
  maxInterval: uint,
  currentTick: uint,

  ring: Vec<Node<T>>
}

impl<T> Iterator<Node<T>> for WheelTimer<T> {
  fn next(&mut self) -> Option<Node<T>> {
    let ticked = self.tick();
    return match ticked {
      Cons(_, _) => Some(ticked),
      Nil => None
    };
  }
}

impl<T> WheelTimer<T> {

  // Creates a new timer with the specified max interval
  fn new(maxInterval: uint) -> WheelTimer<T> {
    // Initialize the ring with Nil values
    let mut ring = Vec::with_capacity(maxInterval);
    for _ in range(0u, maxInterval) {
      ring.push(Nil)
    }

    return WheelTimer{
      maxInterval: maxInterval,
      currentTick: 0,
      ring: ring
    }
  }

  // Returns the number of items currently scheduled
  fn size(&self) -> uint {
    return self.ring.iter().map(|node| node.len()).sum()
  }

  // Schedules a new value, available after `ticks`
  fn schedule(&mut self, ticks: uint, value: T) {
    // Compute the scheduled position in the wheel
    let index = (self.currentTick + ticks) % self.maxInterval;

    // Get the current node at `index` in the wheel
    let node = mem::replace(self.ring.get_mut(index), Nil);

    // Set the position in the wheel with the appended node
    *self.ring.get_mut(index) = node.prepend(value);
  }

  // Tick the timer, returning the list of nodes at the spot
  fn tick(&mut self) -> Node<T> {
    // Get the node at the current tick in the wheel
    let node = mem::replace(self.ring.get_mut(self.currentTick), Nil);

    // Increment the timer
    self.currentTick = (self.currentTick + 1) % self.maxInterval;

    // Return the node that was in that spot
    return node
  }
}

#[test]
fn wheel_timer_new_test() {
  let timer = WheelTimer::<uint>::new(3);
  assert!(timer.maxInterval == 3);
  assert!(timer.ring.capacity() == 3);
  assert!(timer.ring.len() == 3);
}

#[test]
fn wheel_timer_schedule_test() {
  let mut timer = WheelTimer::<&'static str>::new(10);
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
  let mut timer = WheelTimer::<uint>::new(10);

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
  let mut timer = WheelTimer::<uint>::new(10);

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
  let mut timer = WheelTimer::<uint>::new(maxInterval);

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
  let mut timer = WheelTimer::<uint>::new(maxInterval);
  let mut i = 0;

  b.iter(|| {
    timer.schedule(i%maxInterval, i%maxInterval);
    i = i + 1;
  });
}

#[bench]
fn bench_wheel_timer_fast(b: &mut Bencher) {
  let maxInterval = 2;
  let mut timer = WheelTimer::<uint>::new(maxInterval);
  let mut i = 0;

  b.iter(|| {
    timer.schedule(i%maxInterval, i%maxInterval);
    timer.tick();
    i = i + 1;
  });
}
