#![feature(phase)]

#[phase(plugin)]
extern crate lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBER: uint = times_two(3);
    static ref ARRAY_BOXES: [Box<uint>, ..3] = [box 1, box 2, box 3];
    static ref STRING: String = "hello".to_string();
    static ref HASHMAP: HashMap<uint, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0u, "abc");
        m.insert(1, "def");
        m.insert(2, "ghi");
        m
    };
    static ref UNUSED: () = ();
}

fn times_two(n: uint) -> uint {
    n * 2
}

#[test]
fn test_basic() {
    assert_eq!(STRING.as_slice(), "hello");
    assert_eq!(*NUMBER, 6);
    assert!(HASHMAP.find(&1).is_some());
    assert!(HASHMAP.find(&3).is_none());
    assert_eq!(ARRAY_BOXES.as_slice(), &[box 1, box 2, box 3]);
}

#[test]
fn test_repeat() {
    assert_eq!(*NUMBER, 6);
    assert_eq!(*NUMBER, 6);
    assert_eq!(*NUMBER, 6);
}
