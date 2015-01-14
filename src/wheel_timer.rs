#![allow(unstable)]

use std::mem;
use std::ops::IndexMut;

// Simple hashed wheel timer with bounded interval
// See http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf
pub struct WheelTimer<T> {
  max_interval: usize,
  current_tick: usize,
  size: usize,

  ring: Vec<Vec<T>>
}

impl<T> Iterator for WheelTimer<T> {
  type Item = Vec<T>;

  fn next(&mut self) -> Option<Vec<T>> {
    let size = self.size();
    return if size > 0 {
      Some(self.tick())
    } else {
      None
    };
  }
}

impl<T> WheelTimer<T> {

  // Creates a new timer with the specified max interval
  pub fn new(max_interval: usize) -> WheelTimer<T> {
    // Initialize the ring with Nil values
    let mut ring = Vec::with_capacity(max_interval);
    for _ in range(0us, max_interval) {
      ring.push(Vec::new())
    }

    return WheelTimer{
      max_interval: max_interval,
      current_tick: 0,
      ring: ring,
      size: 0,
    }
  }

  // Returns the number of items currently scheduled
  pub fn size(&self) -> usize {
    self.size
  }

  // Schedules a new value, available after `ticks`
  pub fn schedule(&mut self, ticks: usize, value: T) {
    // Compute the scheduled position in the wheel
    let index = (self.current_tick + ticks) % self.max_interval;

    // Get the current node at `index` in the wheel and append the new node
    self.ring.index_mut(&index).push(value);

    // Increment the size counter
    self.size = self.size + 1;
  }

  // Tick the timer, returning the node at the current tick
  pub fn tick(&mut self) -> Vec<T> {
    // Get the node at the current tick in the wheel
    let node = mem::replace(self.ring.index_mut(&self.current_tick), Vec::new());

    // Increment the timer
    self.current_tick = (self.current_tick + 1) % self.max_interval;

    // Reduce the size by the length of the removed node
    self.size = self.size - node.len();

    // Return the node that was in that spot
    return node
  }
}
