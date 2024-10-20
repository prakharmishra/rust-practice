use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    test_hashmap_string();
    test_hashmap_int();
}

fn test_hashmap_string() {
    let mut string_map: HashMap<String, String> = HashMap::new();
    string_map.insert(String::from("one"), String::from("first"));
    string_map.insert(String::from("two"), String::from("second"));
    string_map.insert(String::from("three"), String::from("third"));

    println!("{}", string_map.get("one").expect("one is not present"));
    println!("{}", string_map.get("two").expect("two is not present"));
    println!("{}", string_map.get("three").expect("three is not present"));
}

fn test_hashmap_int() {
    let mut int_map = HashMap::new();
    int_map.insert(1, "one");
    int_map.insert(2, "two");
    int_map.insert(3, "three");

    println!("int map: {:?}", int_map);
}