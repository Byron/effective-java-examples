
pub enum Figure {
    Rectangle { width: f64, height: f64 },
    Square { length: f64 },
    Circle { radius: f64 },
}

pub trait Area {
    fn area(&self) -> f64;
}

pub trait Volume {
    fn volume(&self) -> f64;
}

impl Area for Figure {
    fn area(&self) -> f64 {
        use Figure::*;
        use std::f64::consts::PI;

        match *self {
            Rectangle { width: w, height: h } 
                => w * h,
            Square { length: l }
                => l * l,
            Circle { radius }
                => (radius * radius) * PI
        }
    }
}


mod tests {
    use super::*;
    use std::f64::consts::PI;

    struct FooFigure;

    impl Area for FooFigure {
        fn area(&self) -> f64 {
            42.0
        }
    }

    impl Volume for FooFigure {
        fn volume(&self) -> f64 {
            42.0 * 42.0 * 42.0
        }
    }

    #[test]
    fn usage() {
        fn monomorphic_area<T: Area>(item: &T) -> f64 {
            item.area()
        }

        fn polymorphic_area(item: &Area) -> f64 {
            item.area()
        }

        for &(ref item, want_area) in &[(Figure::Rectangle {width: 3.0, height: 2.0 }, 6.0),
                      (Figure::Square { length: 4.0 }, 16.0),
                      (Figure::Circle { radius: 1.0 }, PI),] {
            assert_eq!(item.area(), want_area);
            assert_eq!(monomorphic_area(item), want_area);
            assert_eq!(polymorphic_area(item), want_area);
        }

        let f = FooFigure;
        assert_eq!(monomorphic_area(&f), 42.0);
        assert_eq!(polymorphic_area(&f), 42.0);
    }

    #[test]
    fn usage2() {
        trait AreaVolume: Area + Volume {}
        impl AreaVolume for FooFigure {}

        fn monomorphic_computation<T: Area + Volume>(item: &T) -> f64 {
            item.area() * item.volume()
        }

        fn polymorphic_computation(item: &AreaVolume) -> f64 {
            item.area() * item.volume()
        }

        // monomorphic_computation(&Figure::Circle { radius: 1.0 });
        monomorphic_computation(&FooFigure);
        polymorphic_computation(&FooFigure);
    }
}