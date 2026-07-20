#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    orig: VecDeque<T>,
    q_min: VecDeque<T>
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self { orig: VecDeque::new(), q_min: VecDeque::new() }
    }

    pub fn push(&mut self, val: T) {
        self.orig.push_back(val.clone());
        while self.q_min.back().is_some_and(|x| val < *x) {
            self.q_min.pop_back();
        }
        self.q_min.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        let x = self.orig.pop_front()?;
        if self.q_min.front().is_some_and(|m| *m == x) {
            self.q_min.pop_front();
        }
        Some(x)
    }

    pub fn front(&self) -> Option<&T> {
        self.orig.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.q_min.front()
    }

    pub fn len(&self) -> usize {
        self.orig.len()
    }

    pub fn is_empty(&self) -> bool {
        self.orig.is_empty()
    }
}
