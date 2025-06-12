// async-await syntax allows to write functions, closures and blocks that can pause execution
// and yield control back to the runtime.
// typically these pauses are to way for IO.


// async under the hood
// Future a state machine which will be polled, to check if it can return a value.
// if the poll fn returns "Ready", a call back function(wake) is called

// Futures are lazy, means they do nothing till they are polled and it is zero cost abstraction.
// Adding await to async function, will draw the Future to completion.

// trait Future {
//     type Output;
//     fn poll(&mut slef, wake: fn()) -> Poll<Self::Output>
// }

// enum Poll<T> {
//     Ready(T),
//     Pending
// }

// async functions are the functions that returns something which implements Future trait.
// fn my_function() -> impl Future<output = ()> {
//     println!("I am async function")
// }

async fn my_function() {
    println!("I am async function");
    let s1 = read_from_db().await;
    println!("First: {}", s1);

    let s2 = read_from_db().await;
    println!("Second: {}", s2);
}

async fn read_from_db() -> String {
    "DB Result".to_owned()
}

// Main is not allowed to be async, need a runtime to run async code for top level function.
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    my_function().await;
}

// Futures can be drawn to completion br 2 ways:
// 1. Using await => only works inside another future(async function)
// 2. Manually polling until it is ready. => for a topmost code, this wil be done by runtime.
