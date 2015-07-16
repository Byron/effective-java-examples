#[test]
/// It's not trivial to properly implement a singleton, and Rust will put all kinds of road-blocks
/// into your way to remind you of it.
/// A mutable singleton is somewhat restricted and requires unsafe code to make work.
/// Last but not least, one way of implementing it could possibly be seen in 
/// `io::stdin()|io::stdout()|io::stderr()` respectively.
fn singletons_in_rust_are_difficult() {
}
