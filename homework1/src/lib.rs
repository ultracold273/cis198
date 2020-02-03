#![allow(dead_code)]

// Uncomment these to have Rust compile the other files as well.
mod lib2;
mod lib3;

// Part 1. Implementing Functions. Taken from Fall 2016's Rust class.
// Write unit tests for you functions.

// Problem 1
// Implement the sum function on slices. Do not use the predefined sum function.
fn sum(slice: &[i32]) -> i32 {
    slice.iter().fold(0, |s, x| s + x)
}

#[test]
fn sum_tests() {
    let slice = [1, 2, 3, 4, 5];
    assert_eq!(sum(&slice), 15);
    assert_eq!(sum(&slice[0..3]), 6);
    assert_eq!(sum(&slice[2..3]), 3);
}

// Problem 2.
// Make unique. Create a new vector which contains each item in the vector
// only once! Much like a set would.
// Please implement this using a for loop.
fn unique(vs: &Vec<i32>) -> Vec<i32> {
    let mut xs = Vec::new();
    for v in vs {
        if !xs.contains(v) {
            xs.push(*v);
        }
    }
    xs
}

#[test]
fn unique_tests() {
    let vs = vec![1, 3, 4, 5, 6, 5, 3, 7];
    assert_eq!(unique(&vs), vec![1, 3, 4, 5, 6, 7]);
}

// Problem 3.
// return a new vector containing only elements that satisfy `pred`.
fn filter(vs: & Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut xs = Vec::new();
    for v in vs {
        if pred(*v) { xs.push(*v) }
    }
    xs
}

#[test]
fn filter_tests(){
    assert_eq!(filter(& vec![1, 2, 3, 4, 5, 6], & |n| n % 2 == 0),
              vec![2, 4, 6]);
}


// Problem 4
// Given starting fibonacci numbers n1 and n2, compute a vector
// where v[i] is the ith fibonacci number.
fn fibonacci(n1: i32, n2: i32, how_many: usize) -> Vec<i32> {
    let mut vs = Vec::new();
    if how_many == 1 {
        vs.push(n1);
    } else if how_many == 2 {
        vs.push(n1);
        vs.push(n2);
    } else {
        vs.push(n1);
        vs.push(n2);
        for i in 2..how_many { vs.push(vs[i-2] + vs[i-1]); }
    }
    vs
}

#[test]
fn fibonacci_tests() {
    assert_eq!(fibonacci(1, 1, 1), vec![1]);
    assert_eq!(fibonacci(1, 1, 2), vec![1, 1]);
    assert_eq!(fibonacci(1, 1, 4), vec![1, 1, 2, 3]);
    assert_eq!(fibonacci(10, 12, 3), vec![10, 12, 22]);
}

// Problem 5
// Create a function which concats 2 strs and returns a String.
// You may use any standard library function you wish.
fn str_concat(s1: &str, s2: &str) -> String {
    let mut s = s1.to_string();
    s.push_str(s2);
    s
}

#[test]
fn str_concat_tests() {
    assert_eq!(str_concat("hello ", "world"), "hello world");
}

// Problem 6
// Create a function which concats 2 string and returns a String.
// You may use any standard library function you wish.
fn string_concat(s1: &String, s2: &String) -> String {
    let mut s = s1.clone();
    s.push_str(s2);
    s
}

#[test]
fn string_concat_tests() {
    let s1 = "hello ".to_string();
    let s2 = "world".to_string();
    assert_eq!(string_concat(&s1, &s2), "hello world");
}

// Problem 7
// Convert a Vec<String> into a Vec<u64>. Assume all strings
// are correct numbers! We will do error handling later. Use
// `.expect("ignoring error")` to ignore Result from parse()
// See https://doc.rust-lang.org/std/primitive.str.html#method.parse
// Use turbo fish! Do not use type inference for parse()
fn concat_all(v: Vec<String>) -> Vec<u64> {
    let mut us = Vec::new();
    for s in v {
        us.push(s.parse::<u64>().expect("ignoring error"));
    }
    us
}

// Implement concat_all using map, parse (with turbo fish), and collect()
// Check out how the lecture does something similar:
// https://github.com/upenn-cis198/lecture2/blob/f54753527c1dabbd5e55c2f48a19745768769beb/src/lib.rs#L362
fn concat_all_with_map(v: Vec<String>) -> Vec<u64> {
    v.iter()
    .map(|s| s.parse::<u64>().expect("ignoring error"))
    .collect()
}

#[test]
fn concat_all_tests() {
    let vs1: Vec<String> = ["33", "44", "55"].iter()
                                .map(|x| x.to_string()).collect();
    let vs2 = vs1.clone();
    assert_eq!(concat_all(vs1), vec![33u64, 44, 55]);
    assert_eq!(concat_all_with_map(vs2), vec![33u64, 44, 55]);
}
