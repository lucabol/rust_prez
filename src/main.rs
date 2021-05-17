#![allow(dead_code, unused_variables)]

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

    {
        let title = ">-> Lifetimes intro";
        fn one_of(s: &str) -> &str {
            s
        }
        let s1 = "Bob".to_owned();
        let s2 = "Rob".to_owned();
        let r = one_of(&s1);
        println!("{}{}", r, s1);
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
