#[test]
/// This item is a collection of naming rules. Let's look at each one in isolation:
/// * **Choose method names carefully**
///   - Dito
/// * **Don't go overboard in providing convenience methods**
///   - Unless you can implement them in terms of required (trait-)methods.
///     Java-interfaces (before java 8) cannot have default-implementations, which
///     requires a workaround to make this happen anyway.
/// * **Avoid long parameter lists**
///   - Dito
/// * **For parameter types, favor interfaces over classes**
///    - In Rust generics are used to make routines flexible. Conversion traits
///      can be used to shift conversions (like AsRef/Into) into the API implementation,
///      making it very comfortable for clients to use.
/// * **Prefer (two-element) enum types to boolean parameters**
///    - It's generally advised to do that. See also: The QT API guide.
fn rulez() {
}
