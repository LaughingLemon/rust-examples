enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let x = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
    let y = Cons(7, Rc::clone(&x));
    let z = Cons(12, Rc::clone(&x));
}

