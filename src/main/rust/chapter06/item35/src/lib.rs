#[test]
/// In Rust, there is no reflection, which is why there are no run-time accessible
/// annoations.
/// However, annotations are commonly added to the AST, and are used by the compiler 
/// to perform dynamic compilation based on variables or external information.
/// Meta-data is attached to items (which is anything, like structs, variables, functions)
/// and be used by compiler plugins or syntax-aware code-generators. Even though compiler plugins
/// are only working on nightly as the compiler API is not stabilized, there are workarounds
/// like `syntex` which allow to manipulate the AST to your liking and output it as source
/// to be compiled by the stable compiler.
fn it_works() {
}
