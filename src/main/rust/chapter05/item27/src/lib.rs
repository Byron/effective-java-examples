#[test]
fn unary_closure() {
    fn apply<T>(item: T) -> T {
        item
    }

    assert_eq!(apply(1), 1);
    assert_eq!(apply("jute"), "jute");

    // How to bring numbers of varying types onto the heap into the same array 
    // without using a enum ?
}

#[test]
fn type_inference() {
    use std::collections::HashMap;

    let anagrams: HashMap<String, Vec<String>> = HashMap::new();
    println!("{:#?}", anagrams);

    let mut anagrams = HashMap::new();
    anagrams.insert("a", vec!["a"]);
}

#[test]
fn max_computation() {
    // doesn't actually work in Java example either ... 
}

#[test]
fn union() {
    use std::collections::BTreeSet;
    let guys: BTreeSet<_> = vec!["Tom", "Dick", "Harry"].into_iter().collect();
    let stooges: BTreeSet<_> = vec!["Larry", "Moe", "Curly"].into_iter().collect();

    println!("{:#?}", guys.union(&stooges).collect::<Vec<_>>());
}