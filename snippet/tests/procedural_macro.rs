use btreemap_proc::btreemap_proc;
use std::collections::BTreeMap;

#[test]
fn test_proc_macro() {
    let map: BTreeMap<&str, &str> = btreemap_proc! {
        "a" => "snippet1",
        "b" => "snippet2",
    };

    assert_eq!(map.get("a"), Some(&"snippet1"));
    assert_eq!(map.get("b"), Some(&"snippet2"));
}
