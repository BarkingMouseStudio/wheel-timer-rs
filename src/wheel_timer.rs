use std::mem;
use std::fmt::{Show, Formatter, Result};
use std::iter::AdditiveIterator;

pub enum Node<T> {
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
  pub fn new(maxInterval: uint) -> WheelTimer<T> {
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
  pub fn size(&self) -> uint {
    return self.ring.iter().map(|node| node.len()).sum()
  }

  // Schedules a new value, available after `ticks`
  pub fn schedule(&mut self, ticks: uint, value: T) {
    // Compute the scheduled position in the wheel
    let index = (self.currentTick + ticks) % self.maxInterval;

    // Get the current node at `index` in the wheel
    let node = mem::replace(self.ring.get_mut(index), Nil);

    // Set the position in the wheel with the appended node
    *self.ring.get_mut(index) = node.prepend(value);
  }

  // Tick the timer, returning the list of nodes at the spot
  pub fn tick(&mut self) -> Node<T> {
    // Get the node at the current tick in the wheel
    let node = mem::replace(self.ring.get_mut(self.currentTick), Nil);

    // Increment the timer
    self.currentTick = (self.currentTick + 1) % self.maxInterval;

    // Return the node that was in that spot
    return node
  }
}
