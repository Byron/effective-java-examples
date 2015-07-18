   Compiling item08 v0.1.0 (file:///Users/byron/Documents/dev/java/effective-java-examples/src/main/rust/chapter03/item08)
#![feature(no_std)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
mod composition {
    #[prelude_import]
    use std::prelude::v1::*;

    struct Point {
        x: i32,
        y: i32,
    }
    #[automatically_derived]
    impl ::std::hash::Hash for Point {
        fn hash<__H: ::std::hash::Hasher>(&self, __arg_0: &mut __H) -> () {
            match *self {
                Point { x: ref __self_0_0, y: ref __self_0_1 } => {
                    ::std::hash::Hash::hash(&(*__self_0_0), __arg_0);
                    ::std::hash::Hash::hash(&(*__self_0_1), __arg_0);
                }
            }
        }
    }
    #[automatically_derived]
    impl ::std::cmp::Eq for Point {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            match *self {
                Point { x: ref __self_0_0, y: ref __self_0_1 } => {
                    (*__self_0_0).assert_receiver_is_total_eq();
                    (*__self_0_1).assert_receiver_is_total_eq();
                }
            }
        }
    }
    #[automatically_derived]
    impl ::std::cmp::PartialEq for Point {
        #[inline]
        fn eq(&self, __arg_0: &Point) -> bool {
            match *__arg_0 {
                Point { x: ref __self_1_0, y: ref __self_1_1 } =>
                match *self {
                    Point { x: ref __self_0_0, y: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Point) -> bool {
            match *__arg_0 {
                Point { x: ref __self_1_0, y: ref __self_1_1 } =>
                match *self {
                    Point { x: ref __self_0_0, y: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }

    enum Color { RED, ORANGE, YELLOW, GREEN, BLUE, INDIGO, VIOLET, }
    #[automatically_derived]
    impl ::std::hash::Hash for Color {
        fn hash<__H: ::std::hash::Hasher>(&self, __arg_0: &mut __H) -> () {
            match (&*self,) {
                (&Color::RED,) => {
                    ::std::hash::Hash::hash(&0usize, __arg_0);
                }
                (&Color::ORANGE,) => {
                    ::std::hash::Hash::hash(&1usize, __arg_0);
                }
                (&Color::YELLOW,) => {
                    ::std::hash::Hash::hash(&2usize, __arg_0);
                }
                (&Color::GREEN,) => {
                    ::std::hash::Hash::hash(&3usize, __arg_0);
                }
                (&Color::BLUE,) => {
                    ::std::hash::Hash::hash(&4usize, __arg_0);
                }
                (&Color::INDIGO,) => {
                    ::std::hash::Hash::hash(&5usize, __arg_0);
                }
                (&Color::VIOLET,) => {
                    ::std::hash::Hash::hash(&6usize, __arg_0);
                }
            }
        }
    }
    #[automatically_derived]
    impl ::std::cmp::Eq for Color {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            match (&*self,) {
                (&Color::RED,) => { }
                (&Color::ORANGE,) => { }
                (&Color::YELLOW,) => { }
                (&Color::GREEN,) => { }
                (&Color::BLUE,) => { }
                (&Color::INDIGO,) => { }
                (&Color::VIOLET,) => { }
            }
        }
    }
    #[automatically_derived]
    impl ::std::cmp::PartialEq for Color {
        #[inline]
        fn eq(&self, __arg_0: &Color) -> bool {
            match (&*self, &*__arg_0) {
                (&Color::RED, &Color::RED) => true,
                (&Color::ORANGE, &Color::ORANGE) => true,
                (&Color::YELLOW, &Color::YELLOW) => true,
                (&Color::GREEN, &Color::GREEN) => true,
                (&Color::BLUE, &Color::BLUE) => true,
                (&Color::INDIGO, &Color::INDIGO) => true,
                (&Color::VIOLET, &Color::VIOLET) => true,
                _ => {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as i32;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as i32;
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &Color) -> bool {
            match (&*self, &*__arg_0) {
                (&Color::RED, &Color::RED) => false,
                (&Color::ORANGE, &Color::ORANGE) => false,
                (&Color::YELLOW, &Color::YELLOW) => false,
                (&Color::GREEN, &Color::GREEN) => false,
                (&Color::BLUE, &Color::BLUE) => false,
                (&Color::INDIGO, &Color::INDIGO) => false,
                (&Color::VIOLET, &Color::VIOLET) => false,
                _ => {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as i32;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as i32;
                    true
                }
            }
        }
    }

    struct ColorPoint {
        point: Point,
        color: Color,
    }
    #[automatically_derived]
    impl ::std::hash::Hash for ColorPoint {
        fn hash<__H: ::std::hash::Hasher>(&self, __arg_0: &mut __H) -> () {
            match *self {
                ColorPoint { point: ref __self_0_0, color: ref __self_0_1 } =>
                {
                    ::std::hash::Hash::hash(&(*__self_0_0), __arg_0);
                    ::std::hash::Hash::hash(&(*__self_0_1), __arg_0);
                }
            }
        }
    }
    #[automatically_derived]
    impl ::std::cmp::Eq for ColorPoint {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            match *self {
                ColorPoint { point: ref __self_0_0, color: ref __self_0_1 } =>
                {
                    (*__self_0_0).assert_receiver_is_total_eq();
                    (*__self_0_1).assert_receiver_is_total_eq();
                }
            }
        }
    }
    #[automatically_derived]
    impl ::std::cmp::PartialEq for ColorPoint {
        #[inline]
        fn eq(&self, __arg_0: &ColorPoint) -> bool {
            match *__arg_0 {
                ColorPoint { point: ref __self_1_0, color: ref __self_1_1 } =>
                match *self {
                    ColorPoint { point: ref __self_0_0, color: ref __self_0_1
                    } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ColorPoint) -> bool {
            match *__arg_0 {
                ColorPoint { point: ref __self_1_0, color: ref __self_1_1 } =>
                match *self {
                    ColorPoint { point: ref __self_0_0, color: ref __self_0_1
                    } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }




}


mod string {
    #[prelude_import]
    use std::prelude::v1::*;
    use std::ascii::AsciiExt;

    fn eq_str_case_insensitive(lhs: &str, rhs: &str) -> bool {
        lhs.eq_ignore_ascii_case(rhs)
    }


}
