#![allow(dead_code, unused_variables, unused_imports)]
use core::cell::{Cell, RefCell};
use rand::Rng;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let k = ();

    {
        let disclaimer = "I don't know what I am talking about.";
        let highlights = [
            "No garbage collection", // vs Go
            "No exceptions",         // wth Go
            "Compiles to LLVM as CLang",
            "As fast & as small as C", // ref clbg
            "No memory safety issues", // ref chrome & msft
            "No concurrency problems",
            "People love it", // ref stackoverflow
        ];
        let lowlights = [
            "Very long (feels infinite) learning curve",
            "Hard to refactor", // ref extract method
            "Not for prototyping",
            "Not mature echosystem",
            "Takes forever to compile",
        ];
    }

    // {
    //     let title = ">-> Copy";
    //     let x = 3;
    //     let y = x;
    //     println!("{} {}", x, y);
    // }

    // {
    //     let title = ">-> Non Copy";
    //     let x = String::from("Bob");
    //     let y = x;
    //     println!("{} {}", x, y);
    // }

    // {
    //     let title = ">-> Function takes ownership";
    //     fn display(s: String) -> () {}
    //     let x = String::from("Bob");
    //     display(x);
    //     println!("{}", x);
    // }

    // {
    //     let title = ">-> Mutable reference";
    //     fn upper(s: &mut String) -> () {
    //         *s = s.to_uppercase();
    //     }
    //     let mut x = String::from("Bob");
    //     upper(&mut x);
    //     println!("{}", x);
    // }

    // {
    //     let title = ">-> Lifetimes intro";
    //     fn one_of(s: &str, t: &str) -> &str {
    //         let mut rng = rand::thread_rng();
    //         let y: f64 = rng.gen();
    //         if y < 0.5 {
    //             s
    //         } else {
    //             t
    //         }
    //     }
    //     let s1 = "Bob".to_owned();
    //     let s2 = "Rob".to_owned();
    //     let r = one_of(&s1, &s2);
    //     println!("res: {} from: {} or: {}", r, s1, s2);
    // }

    // {
    //     let title = ">-> An integer property";
    //     struct Person<'a> {
    //         age0: i32,
    //         age1: &'a i32,
    //         age2: &'a mut i32,
    //         age3: Box<i32>, // single owner, checked at compile time, abstracts heap allocation
    //         age7: Cell<i32>, // interior mutability for Copy types, checked compile time, multiple mutable borrows
    //         age4: RefCell<i32>, // single owner, checked at runtime, mutable & immutable borrows non-copy types
    //         age5: Rc<i32>, // multiple owner ref counted,leaks if cycles, immutable borrows checked at compile time
    //         age6: Arc<i32>, // multiple owner, thread safe
    //     }
    // }

    // {
    //     let title = ">-> Write a function to swap two ints";
    //     fn swap(i: &mut i32, j: &mut i32) {
    //         let tmp = *i;
    //         *i = *j;
    //         *j = tmp;
    //     }
    //     let mut i1 = 1;
    //     let mut i2 = 2;
    //     swap(&mut i1, &mut i2);
    //     println!("{}{}", i1, i2);
    // }

    // {
    //     let title = ">-> Write a function to swap two strings";
    //     fn swap(i: &mut str, j: &mut str) {
    //         let tmp = *i;
    //         *i = *j;
    //         *j = tmp;
    //     }
    //     let mut s1 = "Bob".to_owned();
    //     let mut s2 = "Rob".to_owned();
    //     swap(&mut s1, &mut s2);
    //     std::mem::swap(&mut s1, &mut s2);
    //     println!("{}{}", s1, s2);
    // }

    let _ = ();
}
