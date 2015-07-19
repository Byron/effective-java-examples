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

    pub fn iter<'a>(&'a self) -> Iter<'a> {
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

pub struct Iter<'a> {
    state: u8,
    inner: &'a Foo
}

impl<'a> Iter<'a> {
    fn new(foo: &'a Foo) -> Iter<'a> {
        Iter {
            state: 0,
            inner: foo
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            0 => {
                self.state += 1;
                Some(self.inner.a)
            }
            1 => {
                self.state += 1;
                Some(self.inner.b)
            }
            _ => None
        }
    }
}
