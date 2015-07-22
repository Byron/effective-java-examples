
#[derive(PartialOrd, Ord, Eq, PartialEq)]
pub enum TextStyle {
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

#[test]
fn style() {
    use std::collections::BTreeSet;
    use std::iter::FromIterator;

    let m = BTreeSet::from_iter(vec![TextStyle::Bold, TextStyle::Italic]);
    assert!(m.contains(&TextStyle::Bold));
    assert!(m.contains(&TextStyle::Italic));
    assert!(!m.contains(&TextStyle::Underline));
}