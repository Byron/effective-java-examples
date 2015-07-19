#![feature(hashmap_hasher)]

use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::collections::hash_state::HashState;
use std::hash::Hash;
use std::cmp::Eq;

pub struct InstrumentedSet<T, S = RandomState> {
    inner: HashSet<T, S>,
    add_count: usize,
}

impl<T, S> InstrumentedSet<T, S>
where T: Eq + Hash, 
      S: HashState {
        
    pub fn new(set: HashSet<T, S>) -> InstrumentedSet<T, S> {
        InstrumentedSet {
            inner: set,
            add_count: 0,
        }
    }

    pub fn add_count(&self) -> usize {
        self.add_count
    }

    pub fn add(&mut self, item: T) -> bool {
        if self.inner.insert(item) == true {
            self.add_count += 1;
            true
        } else {
            false
        }
    }

    pub fn add_all<I>(&mut self, it: I) 
    where I: IntoIterator<Item = T> {
        for item in it.into_iter() {
            if self.inner.insert(item) == true {
                self.add_count += 1
            }
        }
    }
}