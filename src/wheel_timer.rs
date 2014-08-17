use std::mem;

// Simple hashed wheel timer with bounded interval
// See http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf
pub struct WheelTimer<T> {
  maxInterval: uint,
  currentTick: uint,
  size: uint,

  ring: Vec<Vec<T>>
}

impl<T> Iterator<Vec<T>> for WheelTimer<T> {
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
  pub fn new(maxInterval: uint) -> WheelTimer<T> {
    // Initialize the ring with Nil values
    let mut ring = Vec::with_capacity(maxInterval);
    for _ in range(0u, maxInterval) {
      ring.push(Vec::new())
    }

    return WheelTimer{
      maxInterval: maxInterval,
      currentTick: 0,
      ring: ring,
      size: 0,
    }
  }

  // Returns the number of items currently scheduled
  pub fn size(&self) -> uint {
    self.size
  }

  // Schedules a new value, available after `ticks`
  pub fn schedule(&mut self, ticks: uint, value: T) {
    // Compute the scheduled position in the wheel
    let index = (self.currentTick + ticks) % self.maxInterval;

    // Get the current node at `index` in the wheel and append the new node
    self.ring.get_mut(index).push(value);

    // Increment the size counter
    self.size = self.size + 1;
  }

  // Tick the timer, returning the node at the current tick
  pub fn tick(&mut self) -> Vec<T> {
    // Get the node at the current tick in the wheel
    let node = mem::replace(self.ring.get_mut(self.currentTick), Vec::new());

    // Increment the timer
    self.currentTick = (self.currentTick + 1) % self.maxInterval;

    // Reduce the size by the length of the removed node
    self.size = self.size - node.len();

    // Return the node that was in that spot
    return node
  }
}
