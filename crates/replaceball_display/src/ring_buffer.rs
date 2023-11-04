use std::collections::{VecDeque, vec_deque::Iter};

#[derive(Debug, Clone)]
pub struct RingBuffer<T> {
    buffer: VecDeque<T>,
    max_size: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_size),
            max_size
        }
    }

    /// Adds elt to the back of the ring buffer. If the buffer is full, will
    /// pop the front element off and return it. Otherwise, returns None.
    pub fn push_back(&mut self, elt: T) -> Option<T> {
        self.buffer.push_back(elt);

        if self.buffer.len() > self.max_size {
            self.buffer.pop_front()
        } else {
            None
        }
    }

    /// Adds elt to the front of the ring buffer. If the buffer is full, will
    /// pop the back element off and return it. Otherwise, returns None.
    pub fn push_front(&mut self, elt: T) -> Option<T> {
        self.buffer.push_front(elt);

        if self.buffer.len() > self.max_size {
            self.buffer.pop_back()
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.buffer.iter()
    }
}

