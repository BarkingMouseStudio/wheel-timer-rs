//! Simple hashed wheel timer with bounded interval.
//!
//! Relevant:
//! http://www.cs.columbia.edu/~nahum/w6998/papers/sosp87-timing-wheels.pdf

use std::mem;
use std::ops::IndexMut;

/// A simple wheel timer implementation with a fixed ring size.
pub struct WheelTimer<T> {
    max_interval: usize,
    current_tick: usize,
    size: usize,

    ring: Vec<Vec<T>>,
}

/// Iterator implementation allows for using the wheel timer in a for loop.
///
/// # Example
///
/// ```
/// use wheel_timer::WheelTimer;
///
/// let mut timer: WheelTimer<usize> = WheelTimer::new(20);
/// for result in timer {
///   // result is a vector of the values at that step
/// }
/// ```
impl<T> Iterator for WheelTimer<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let size = self.size();
        if size > 0 {
            Some(self.tick())
        } else {
            None
        }
    }
}

impl<T> WheelTimer<T> {
    /// Creates a new timer with the specified max interval.
    ///
    /// # Example
    ///
    /// ```
    /// use wheel_timer::WheelTimer;
    ///
    /// let mut timer: WheelTimer<usize> = WheelTimer::new(20);
    /// ```
    pub fn new(max_interval: usize) -> WheelTimer<T> {
        // Initialize the ring with Nil values
        let mut ring = Vec::with_capacity(max_interval);
        for _ in 0..max_interval {
            ring.push(Vec::new())
        }

        WheelTimer {
            max_interval,
            current_tick: 0,
            ring,
            size: 0,
        }
    }

    /// Returns the number of items currently scheduled.
    ///
    /// # Example
    ///
    /// ```
    /// use wheel_timer::WheelTimer;
    ///
    /// let mut timer: WheelTimer<usize> = WheelTimer::new(20);
    /// timer.schedule(4, 1);
    /// timer.schedule(7, 1);
    /// timer.schedule(1, 1);
    /// assert_eq!(timer.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Schedules a new value, available after `ticks` have passed.
    ///
    /// # Example
    ///
    /// ```
    /// use wheel_timer::WheelTimer;
    ///
    /// let mut timer: WheelTimer<usize> = WheelTimer::new(20);
    /// timer.schedule(4, 7); // schedule value 7 for 4 ticks
    /// ```
    pub fn schedule(&mut self, ticks: usize, value: T) {
        // Compute the scheduled position in the wheel
        let index = (self.current_tick + ticks) % self.max_interval;

        // Get the current node at `index` in the wheel and append the new node
        self.ring.index_mut(index).push(value);

        // Increment the size counter
        self.size += 1;
    }

    /// Tick the timer, returning the node at the current tick.
    ///
    /// # Example
    ///
    /// ```
    /// use wheel_timer::WheelTimer;
    ///
    /// let mut timer: WheelTimer<usize> = WheelTimer::new(20);
    /// timer.schedule(3, 4); // schedule value 4 for 3 ticks
    /// timer.tick();
    /// timer.tick();
    /// timer.tick();
    /// let result = timer.tick(); // vec![4]
    /// assert_eq!(result.len(), 1);
    /// ```
    pub fn tick(&mut self) -> Vec<T> {
        // Get the node at the current tick in the wheel
        let node = mem::replace(self.ring.index_mut(self.current_tick), Vec::new());

        // Increment the timer
        self.current_tick = (self.current_tick + 1) % self.max_interval;

        // Reduce the size by the length of the removed node
        self.size -= node.len();

        // Return the node that was in that spot
        node
    }
}
