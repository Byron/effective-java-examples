#[test]
fn closure() {
    let mut n = [1, 2, 3, 4, 9, 0];
    n.sort();
    assert_eq!(n[0], 0);
    assert_eq!(n[5], 9);

    n.sort_by(|a, b| b.cmp(a));
    assert_eq!(n[0], 9);
    assert_eq!(n[5], 0);
}