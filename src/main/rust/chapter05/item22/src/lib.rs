use std::borrow::Borrow;

pub struct NonCopyable;

pub struct Foo {
    a: u16,
    b: u16,
}

pub struct Builder {
    inner: Foo,
}

impl Foo {

    pub fn new(a: u16, b: u16) -> Foo {
        Foo {
            a: a,
            b: b,
        }
    }

    pub fn iter(&self) -> Iter<&Self> {
        Iter::new(self)
    }

    pub fn into_iter(self) -> Iter<Self> {
        Iter::new(self)
    }
}

impl Builder {

    pub fn new() -> Self {
        Builder {
            inner: Foo::new(0, 0),
        }
    }

    pub fn a(mut self, v: u16) -> Self {
        self.inner.a = v;
        self
    }

    pub fn b(mut self, v: u16) -> Self {
        self.inner.b = v;
        self
    }

    pub fn build(self) -> Foo {
        return self.inner
    }
}

pub struct Iter<T> where T: Borrow<Foo> {
    state: u8,
    inner: T
}

impl<T> Iter<T> where T: Borrow<Foo> {
    fn new(foo: T) -> Iter<T> {
        Iter {
            state: 0,
            inner: foo
        }
    }
}

impl<T> Iterator for Iter<T> where T: Borrow<Foo> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.state += 1;
                Some(self.inner.borrow().a)
            }
            1 => {
                self.state += 1;
                Some(self.inner.borrow().b)
            }
            _ => None
        }
    }
}
