use std::ops::{Deref, DerefMut};

pub fn copy_memory(input: &[u8], out: &mut [u8]) -> usize {
    for count in 0..input.len() {out[count] = input[count];}
    input.len()
}

pub struct Toggle<T> {
    inner: T,
    on: bool,
}

impl<T> Toggle<T> {
    pub fn on(inner: T) -> Self {
        Self {
            inner: inner,
            on: true
        }
    }

    pub fn off(inner: T) -> Self {
        Self {
            inner: inner,
            on: false
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn enable(&mut self) {
        self.on = true;
    }

    pub fn is_on(&self) -> bool {
        self.on
    }
}

impl<T> Deref for Toggle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for Toggle<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.inner
    }
}
