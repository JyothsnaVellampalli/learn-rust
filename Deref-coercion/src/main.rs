// Deref Coercion: Coerce refernce of one type to reference of other type.
// Happens implicitly on passing references as params to methods.
// Deref Coercion only works on types that implements Deref and DerefMut traits.
// Happens at compile time, so there is no cost during run time.
// Also coerce mutable refernce to immutable reference, whereas opposite doesn't works.

use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T
}

impl<T> MySmartPointer<T> {
    fn new(value: T) ->  MySmartPointer<T>{
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    } 
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    println!("Hello, world!");

    let str1 = Box::new("Hello world!".to_owned());

    let str2 = MySmartPointer::new(Box::new("Hello World!".to_owned()));

    // Deref happened, coerced Box and MySmartpointer to &str.
    print_val(&str1);
    print_val(&str2);
}

fn print_val(val: &str) {
    println!("val: {}", val);
}
