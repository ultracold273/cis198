
// Part 3.
// Lifetimes and move semantics.

// Problem 1.
// What went wrong? Copy strings properly.
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    // let str2 = str1;        // the str1 is moved and can no more be used
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}

// Problem 2.
// Question 2: How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;        // primitive types are inherently cloned
    assert_eq!(i1, i2);
}

// Problem 3.
// These two don't work either. Fix by changing the type of "string" in the function
// copy_me ONLY, and by adjusting the parameter to "copy_me" where it's called.
#[test]
fn eat_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn eat_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}

fn copy_me(string : &String) -> String {
    string.clone()
}

// Problem 4.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
fn new_ref_string() -> &'static String {
    unimplemented!();
}

// Problem 5.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
fn new_ref_str() -> &'static str {
    unimplemented!();
}

// Problem 6.
// Now we know how to implement this type of function. Implement it and write a test
// pick_longest_tests2() which passes all tests.
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

#[test]
fn pick_longest2_tests() {
    assert_eq!(pick_longest2("cat", "dog"), "cat");
    assert_eq!(pick_longest2("c", "abcde"), "abcde");
}

// Problem 7.
// Write a function with a type signature which type checks the following test:
// and passes the test.
// This function compares it's second argument againsts all elements in it's first
// argument, returning those that are less than (<).
fn find_lesser_values<'a>(v: &Vec<&'a str>, s: &str) -> Vec<&'a str> {
    let mut ls = Vec::new();
    for c in v {
        if *c < s {
            ls.push(*c)
        }
    }
    ls
}

#[test]
fn find_lesser_values_test() {
    assert_eq!(find_lesser_values(& vec!["foo", "bar", "foobar"], "zzzzzzzz"),
               vec!["foo", "bar", "foobar"]);
    assert_eq!(find_lesser_values(& vec!["foo", "bar", "foobar"], "bars"),
               vec!["bar"]);
    // Add more tests.
}

// Problem 8
// Move semantics present intersting API design choices not found in other languages.
// HashMap is an example of such a API.
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// Specifically, the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

use std::collections::HashMap;

// Implement the following function which converts pairs from a vector,
// into key-value pairs in a hashmap.

fn vector_to_hashmap(v: &Vec<(i32, String)>) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for t in v {
        map.insert(t.0, t.1.clone());
    }
    map
}

#[test]
fn vector_to_hashmap_tests() {
    let v = vec![(1, "a".to_string()), (3, "c".to_string()), 
                 (9, "i".to_string()), (10, "j".to_string())];
    let mut map = HashMap::new();
    map.insert(1, "a".to_string());
    map.insert(3, "c".to_string());
    map.insert(9, "i".to_string());
    map.insert(10, "j".to_string());
    assert_eq!(vector_to_hashmap(&v), map);
}

// Rust prevents us from deleting entries while iterating... Rewrite
// this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    // for k in h.keys() {
    //     if *k < 0 {
    //         h.remove(k);
    //     }
    // }
    // let ks: Vec<i32> = h.keys().filter(|k| *k < 0).map(|k| *k).collect();
    let mut ks: Vec<i32> = Vec::new();

    for k in h.keys() {
        if *k < 0 {
            ks.push(*k);
        }
    }

    for k in ks {
        h.remove(&k);
    }
}

// For all entries in `add`: (k, v)
// If `k` exists in `merged`, append `v` to the value of `merged[k]`.
// If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.

// Use the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
// Use `or_insert` and `and_modify`.
fn merge_maps(merged: &mut HashMap<String, String>, add: HashMap<String,String>) {
    for k in add.keys() {
        merged.entry(k.clone())
              .and_modify(|s| (*s).push_str(&add[k]))
              .or_insert(add[k].clone());
    }
}
