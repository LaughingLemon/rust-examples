enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let x = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
    println!("refcount creating  x is {}", Rc::strong_count(&x));
    let y = Cons(7, Rc::clone(&x));
    println!("refcount after cloning is {}", Rc::strong_count(&x));
    {
        let z = Cons(12, Rc::clone(&x));
        println!("refcount within scope is {}", Rc::strong_count(&x));
    }
    println!("refcount after scope is {}", Rc::strong_count(&x));
}

