#[test]
/// As there is no inheritance, there is no need to add annotations to indicate 
/// programmer intentions.
/// When implementing a trait, the compiler knows if something has gone wrong,
/// as trait implementations are separate `impl` blocks.
/// Not placing all implementations into the same body seems to be a great way
/// to prevent errors that easily happen in Java, while keeping the code 
/// concise.
fn it_works() {
}
