#![allow(dead_code, unused_variables)]
use core::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

fn mutate(s: &mut String) {
    *s = s.to_uppercase();
}
fn main() {
    {
        let title = ">-> Copy";
        let x = 3;
        let y = x;

        println!("{} {}", x, y);
    }

    // {
    // let title = ">-> Non Copy";
    // let x = String::from("Bob");
    // let y = x;
    // println!("{} {}", x, y);
    // }

    // {
    // let title = ">-> Function takes ownership";
    // fn display(s: String) -> () {}
    // let x = String::from("Bob");
    // display(x);
    // println!("{}", x);
    // }

    // {
    // let title = ">-> Mutable reference";
    // fn upper(s: &mut String) -> () {
    // *s = s.to_uppercase();
    // }
    // let mut x = String::from("Bob");
    // upper(&mut x);
    // println!("{}", x);
    // }

    // {
    // let title = ">-> Lifetimes intro";
    // fn one_of(s: &str, t: &str) -> &str {
    // let mut rng = rand::thread_rng();
    // let y: f64 = rng.gen();
    // if y < 0.5 {
    // s
    // } else {
    // t
    // }
    // }
    // let s1 = "Bob".to_owned();
    // let s2 = "Rob".to_owned();
    // let r = one_of(&s1);
    // println!("{}{}", r, s1);
    // }

    let title = ">-> Other common case of runtime";

    struct Person<'a> {
        age0: i32,
        age1: &'a i32,
        age2: &'a mut i32,
        age3: Box<i32>, // single owner, checked at compile time, abstracts heap allocation
        age7: Cell<i32>, // interior mutability for Copy types, checked compile time, multiple mutable borrows
        age4: RefCell<i32>, // single owner, checked at runtime, mutable & immutable borrows non-copy types
        age5: Rc<i32>, // multiple owner ref counted,leaks if cycles, immutable borrows checked at compile time
        age6: Arc<i32>, // multiple owner, thread safe
    }
    // {
    // let title = ">-> My, oh my";
    // fn swap<'a>(mut s: &'a mut str, r: &'a mut str) {
    // s = r;
    // }
    // let mut s1 = "Bob".to_owned();
    // let mut s2 = "Rob".to_owned();
    // swap(&mut s1, &mut s2);
    // println!("{}{}", s1, s2);
    // }

    let _ = ();
}
