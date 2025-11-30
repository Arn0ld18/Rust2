use snippet::btreemap;

#[test]
fn test_btreemap_macro() {
    let map = btreemap! {
        "x" => 10,
        "y" => 20,
    };

    assert_eq!(map.get("x"), Some(&10));
    assert_eq!(map.get("y"), Some(&20));
}

#[test]
fn test_btreemap_empty() {
    let map: std::collections::BTreeMap<&str, i32> = btreemap! {};
    assert!(map.is_empty());
}
