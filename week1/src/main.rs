use clap::Parser; // Similar to python argparse

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;
use std::collections::VecDeque;

fn main() {
    ex_collections();
    ex_rand();
    ex_vecdeque();
    ex_linkedlist();
    ex_use_of_collect();
}

fn ex_collections() {
    // Immutable by default
    // Sequences <- like a python list
    // Maps <- like a python dictionary
    let fruits = vec!["apple", "banana", "cherry"];
    for fruit in fruits {
        println!("{}", fruit);
    }

    let mut map = std::collections::HashMap::new();
    map.insert("key", "value");
    map.insert("key2", "value2");

    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}

fn ex_rand() {
    let mut fruit: Vec<&str> = vec!["apple", "banana", "cherry", "orange", "pear"];

    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);
    println!("Random fruit:");
    for i in 0..5 {
        println!("{}", fruit[i]);
    }
}

fn ex_vecdeque() {
    /* This function
     * - creates a VecDeque
     * - pushes elements to the front and back
     * - pops elements from the front and back
     * - prints the VecDeque
     * */
    let mut v = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_front(0);
    v.push_front(-1);
    v.push_front(-2);

    println!("Deque: {:?}", v);

    let first = v.pop_front();
    let last = v.pop_back();

    println!("Deque: {:?}", v);
    println!("First: {:?}", first);
    println!("Last: {:?}", last);
}

fn ex_linkedlist() {
    /* This function
     * - creates a LinkedList
     * - pushes elements to the front and back
     * - pops elements from the front and back
     * - prints the LinkedList
     * */
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_front(0);
    list.push_front(-1);
    list.push_front(-2);

    println!("List: {:?}", list);

    let first = list.pop_front();
    let last = list.pop_back();

    println!("List: {:?}", list);
    println!("First: {:?}", first);
    println!("Last: {:?}", last);
}

fn ex_use_of_collect() {
    /* Here we use collect()
     * as it's a convenient way to convert one collection to another
     * */
    let keys = vec!["a", "b", "c"];
    let values = vec![1, 2, 3];
    let collected: std::collections::HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    for (key, value) in &collected {
        println!("{}: {}", key, value);
    }
}