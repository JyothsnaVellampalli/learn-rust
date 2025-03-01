// when programm stored in local disk is executed, a process is created in RAM.
// A process is independent i.e., it has it's own address space and heap in RAM and also copy of prog code, files, ...
// Multiple process => Multiple instances of your program.
// Each process can have one or more threads.
// Thread = sequence of instructions
// threads can be execute concurrently = concurrency 
// A process can spwan multiple threads.
// Threads share the same heap but have their own stack. => Threads are useful when we want 
// to run different parts of thr prog concurrently and talk to each other.

// Scheduler will selct one thread and set it to execution in cpu core and then switch to another thread.
// Premptive scheduling: Each thread will get some time to execute, once that time is up,
// next thread will start to execute. = Time slicing.
// In premptive scheduling, scheduler can switch the thread at any point and the programmer has no control over it.

// If CPU has multiple cores, then multiple threads can run in parllel.

// Cocurrency Models:
// 1. OS Threads - Threads are managed by OS
// 2. Asynchronous Programming - Executing multiple functions at the same time.
// 3. Coroutines - Executing multiple functions at the same time, hide lower level details.
// 4. Event-Driven Programming - In JS
// 5. The Actor Model

// In rust - OS Threads and Asynchronous Programming(Async Tasks)

// OS Threads:
// Managed by OS
// Premptive scheduling
// High performance overhead
// ideal for CPU-bound tasks

// Async Tasks:
// Managed by Rust runtime
// Cooperative scheduling
// Low performance overhead
// Ideal for IO-bound tasks
// code looks like synchronous code but runs asynchronously(concurrently).

// trait Future {
//     type Output;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// Future does nothing till it is polled.
// Full is driven to completion by awaiting or giving it to an executor.

// enum Poll<T> {
//     Ready(T),
//     Pending,
// } 

// fn my_function() -> impl Future<Output = ()> {
//     println!("I am an async function!");
// }

async fn my_function() {
    println!("I am an async function!");
    let s1 = read_from_db().await;
    println!("first, {}", s1);
    let s2 = read_from_db().await;
    println!("second, {}", s2);
}

async fn read_from_db() -> String {
    String::from("hello from DB")
}

// awiat can be used in future inside another future(async function inside another async function)
// for topmost future(in main, ..), we need manual polling = Runtime or executor.
// Rust does not have built in async runtime
// tokio - most popular async runtime library

// attribute macro
#[tokio::main]
async fn main() {
    let res = my_function().await;
    println!("Rust");
}