#[test]
/// As reference counted resources are not the norm in Rust, it's less likely to 
/// keep them alive for too long and thus prevent freeing them.
/// In theory, all the issues mentioned in this item can appear in Rust as well,
/// even though I'd argue that it's less likely to lead to problems just because
/// in Rust, these resource are very explicit.
fn it_works() {
}
