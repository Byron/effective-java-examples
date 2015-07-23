pub struct Object(i32);

impl Object {
    fn doit(&self) -> i32 {
        // Yes, it is possible to protect against 0-attacks, but it's not feasible !
        // If untrusted code is running in your process, it can make you crash, either
        // by calling you or by doing it in his own code.
        // The good thing is that you are protected against accidental 0-references, which
        // simply don't happen outside of unsafe code.
        if self as *const Object as usize == 0 {
            return 0;
        }
        self.0
    }
}

pub fn do_something_with(o: &Object) -> i32 {
    o.doit()
}

#[test]
fn attack() {
    // needs unsafe !
    // do_something_with(&*(0 as *const Object));
    unsafe {
        // this would crash if Object wouldn't be paranoid (and impractical)
        do_something_with(&*(0 as *const Object));
    }
}