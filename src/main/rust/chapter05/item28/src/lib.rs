#[test]
fn explicit_conversions() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.extend(["1", "2", "-3", "0"].iter()
                                        .map(|s| s.parse().unwrap()));

    assert_eq!(numbers, vec![1, 2, -3, 0]);
}