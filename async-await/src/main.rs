// async-await syntax allowa to write functions, closures and blocks that can pause execution
// and yield control back to the runtime.


// async under the hood
// Future a state machine which will be polled, to check if it can return a value.
// if the poll fn returns "Ready", a call back function(wake) is called
// Adding await to async function, will draw the Future to completion.
// trait Future {
//     type Output;
//     fn poll(&mut slef, wake: fn()) -> Poll<Self::Output>
// }

// enum Poll<T> {
//     Ready(T),
//     Pending
// }

// aync functions are the functions that returns something which implements Future trait.
// fn my_function() -> impl Future<output = ()> {
//     println!("I am async function")
// }

async my_function() {
    println!("I am async function")
    let s1 = read_from_db().await;
    println!("First: {}", s1);

    let s2 = read_from_db().await;
    println!("Second: {}", s2);
}

async fn read_from_db() -> String {
    "DB Result".to_owned()
}

fn main() {
    println!("Hello, world!");
    my_function().await;
}
