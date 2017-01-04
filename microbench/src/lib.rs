#![feature(test)]

extern crate test;

use std::collections::HashMap;

pub fn create_hashmap_with_insert() {
    let mut h: HashMap<String, String> = HashMap::new();
    h.insert("hello1".to_string(), "world1".to_string());
    h.insert("hello2".to_string(), "world2".to_string());
    h.insert("hello3".to_string(), "world3".to_string());
    h.insert("hello4".to_string(), "world4".to_string());
    h.insert("hello5".to_string(), "world5".to_string());
}

pub fn create_hashmap_with_tuple_collect() {
    let mut h: HashMap<String, String> = vec![
        ("hello1", "world1"),
        ("hello2", "world2"),
        ("hello3", "world3"),
        ("hello4", "world4"),
        ("hello5", "world5"),
    ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<String, String>>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_create_hashmap_with_insert(b: &mut Bencher) {
        b.iter(|| create_hashmap_with_insert());
    }

    #[bench]
    fn bench_create_hashmap_with_tuple_collect(b: &mut Bencher) {
        b.iter(|| create_hashmap_with_tuple_collect());
    }
}
