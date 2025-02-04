// Box smart pointer is used when transfering large amount of data while transfering ownership.
// Trait objects has to be stored in sometype of pointers.

struct Container {
    name: String,
    // To avoid recursive type, we need to insert struct in some pointer
    container: Box<Container>
}

fn main() {
    println!("Hello, world!");
}
