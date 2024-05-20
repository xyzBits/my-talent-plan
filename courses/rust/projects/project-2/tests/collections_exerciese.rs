use std::cmp::Ordering;
use std::collections::{BTreeMap, VecDeque};
use std::hash::{Hash, Hasher};

#[test]
fn test_iterator() {
    let vec = vec![1, 2, 3, 4];

    for x in vec.iter() {
        println!("vec contained {x:?}");
    }

    let mut vec = vec![1, 2, 3, 4];

    for x in vec.iter_mut() {
        *x += 1
    }
    println!("{:?}", vec);
}

#[test]
fn test_into_iter() {
    let mut vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![10, 20, 30, 40];

    vec1.extend(vec2);

    let vec = vec![1, 2, 3, 4];
    let buf = vec.into_iter().collect::<VecDeque<_>>();
}

#[derive(Debug)]
struct Foo {
    a: u32,
    b: &'static str,
}

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
    }
}

impl Eq for Foo {}

impl Hash for Foo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state)
    }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.a.partial_cmp(&other.a)
    }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.a.cmp(&other.a)
    }
}

#[test]
fn test_map_insert_complex_key() {
    let mut map = BTreeMap::new();

    map.insert(Foo { a: 1, b: "baz" }, 99);

    map.insert(Foo { a: 1, b: "xyz" }, 100);

    assert_eq!(map.values().next().unwrap(), &100);

    assert_eq!(map.keys().next().unwrap().b, "baz");
}