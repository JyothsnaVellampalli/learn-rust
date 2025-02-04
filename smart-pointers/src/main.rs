// Box smart pointer is used when transfering large amount of data while transfering ownership.
// Trait objects has to be stored in sometype of pointers.

// RC smart pointer helps for immutable shared ownership,
// RC pointer is only suitable for single-threaded application
// For multi-threaded application, Atomic RC pointer should be used.

// To mutably share ownership, use RefCell smart pointer.
// This is unsafe as rust compiler(does not obey borrowing rules), memory safety is on developer.

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
