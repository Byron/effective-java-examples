pub struct Foo {
    pub a: u16,
    pub b: u16,
}

impl Foo {
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter::new(self)
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
