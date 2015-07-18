mod composition {

    #[derive(PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(PartialEq, Eq, Hash)]
    enum Color {
        RED, ORANGE, YELLOW, GREEN, BLUE, INDIGO, VIOLET
    }

    #[derive(PartialEq, Eq, Hash)]
    struct ColorPoint {
        point: Point,
        color: Color,
    }

    #[test]
    fn point_equality() {
        use std::collections::HashSet;

        let one = Point { x: 1, y: 2 };
        let two = Point { x: 3, y: 4 };
        let three = Point { x: 1, y: 2 };

        assert!(one == three);
        assert!(one == one);
        assert!(one != two);
        assert!(three != two);

        let mut m = HashSet::new();
        assert!(m.insert(one) == true);
        assert!(m.insert(two) == true);
        assert!(m.insert(three) == false);
    }
}


mod string {
    use std::ascii::AsciiExt;

    fn eq_str_case_insensitive(lhs: &str, rhs: &str) -> bool {
        lhs.eq_ignore_ascii_case(rhs)
    }

    #[test]
    fn case_comparison() {
        assert!(eq_str_case_insensitive("hello", "HellO"));
    }

}