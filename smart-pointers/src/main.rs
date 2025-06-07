// smart pointers are data structures that act like pointers, also have some additional metadata and capabilities.
// reference is a pointer, while references only borrow data, while smart pointers own the data they point to.
// vec, string are smart pointers.
// Smart pointers are usually implemented using structs. this implement deref and drop traits.


// Ability to deref by implementing deref trait.
// Rust substitutes the * operator with a call to the deref method
// Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.

// RC smart pointer allows to have multiple owners to the same data.
// When multiple parts of our prog needs the same data and can't decide which part will finish last.
// only for use in single-threaded scenarios.

// Ref cell smart pointer follows interior mutability design pattern.
// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;
// the pattern uses unsafe code inside a data structure 
// validates borrowing rules at runtime, not at compile time.
// With RefCell<T>, we use the borrow and borrow_mut methods, 

// Rc<T>, RefCell<T> is only for use in single-threaded scenarios

// RC<T> lets you have a multiple owners to the data giving immutable access;
// RefCell<T> lets you have a single owner to the data giving mutable access;
// an Rc<T> that holds a RefCell<T> can have multiple owners and that you can mutate!

// You can also create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade
// Weak references don’t express an ownership relationship, and their count doesn’t affect when an Rc<T> instance is cleaned up. 
// "upgrade" method returns an Option<Rc<T>>, retruns none if RC is already dropped.


// Refernce cycles: create references where items refer to each other in a cycle.
// refernce cycles leads to memory leaks (that is never cleaned up).


// Custom smart pointer.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;

struct Container {
    name: String,
    // To avoid recursive type, we need to insert struct in some pointer
    container: Box<Container>
}

struct Database {
    max_connections: u32,
}

struct ContentService {
    db: Rc<RefCell<Database>>
}

struct AuthService {
    db: Rc<RefCell<Database>>
}

fn main() {
    println!("Hello, world!");

    // Reference count = 1
    let db = Rc::new(RefCell::new(Database{max_connections: 100}));

    // clone in Rc does not clones, but increments the counter.
    let auth_service = AuthService{db: Rc::clone(&db)}; // Reference count = 2
    let content_service = ContentService{db: Rc::clone(&db)}; // Reference count = 3

    // db.max_connections = 200; // Wrap in RefCell for mutable ownership sharing.

    db.borrow_mut().max_connections = 200;
    db.borrow_mut().max_connections = 300;
}