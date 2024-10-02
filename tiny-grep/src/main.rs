mod mybox;
use crate::List::Cons;
use crate::List::Nil;
use mybox::*;

mod rc_learn;
use crate::RcList::RcCons;
use crate::RcList::RcNil;
use rc_learn::*;
use std::rc::Rc;

use std::ops::Deref;

fn main() {
    let point = Box::new(5);
    println!("b = {}", point);
    println!("b = {}", point.deref());
    println!("b = {}", *point);

    println!("box: {:#?}", Box::new(Cons(5, Box::new(Nil))));

    let y = MyBox::new(44);
    assert_eq!(44, *y);
    // *(y.deref())

    /* ========= Rc learn =========== */
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
        // 注意 a.clone() Rc::clone(&a) 区别
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
        println!("c: {:#?}", c);
    }
    println!("count after c drop: {}", Rc::strong_count(&a));
    println!("b: {:#?}", b);
}
