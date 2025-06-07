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


// Creating Threads

// use std::thread;

// fn main() {
//     // we want main thread to wait for the spawned thread finish the execution, this
//     // can be achieved by using JOIN HANDLES.
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("in spawned thread: {}", i);
//         }
//     });

//     for i in 1..10 {
//         println!("in main thread: {}", i);
//     }

//     // if the thread panics,join method wil return a error type.
//     handle.join().unwrap();

//     // Its upto OS how to schedule threads, this is called "Premptive Scheduling".
// }

// Moving valus in to threads 
// Use move keyword to move values into the thread, as we can't know when the thread  will
// finish execution, by that time, the main prog can drop the value thread is referring to.
// so, we need to move values to thread and give ownership to the thread.

// Messaging among threads
// MPSC: Multiple Producer Single Consumer

// use std::{thread, sync::mpsc};

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let sentences = vec![
//         String::from("Hello, world!"),
//         String::from("Rust is awesome!"),
//         String::from("Concurrency is powerful!"),
//     ];

//     for sentence in sentences {
//         // cloning the sender to move to each thread;
//         let tx_clone = tx.clone();

//         thread::spawn(move || {
//             let reversed = sentence.chars().rev().collect::<String>();
//             // send the reversed string to the channel
//             tx_clone.send(reversed).unwrap();
//         });
//     }

//     drop(tx); // Close(drops) the sender to signal no more messages will be sent.

//     // recv() method waits for single message,
//     // and blocks the current thread until all senders are dropped.
//     // rx.recv().expect("Failed to receive message");

//     for sentence in rx {
//         println!("Received: {}", sentence);
//     }
// }


// To share state across threads,
// we use mutex = Mutual exclusion
// Mutex allows only one thread to access the data at a time.
// we have lock and unlock to allow only one thread to mutate the data at a time.
// when lock goes out of scope, it automatically unlocks the mutex.

use std::{thread, sync::{Mutex, Arc}};

struct Database {
    connections: Vec<u32>
}

impl Database {
    fn new() -> Self {
        Database {
            connections: vec![]
        }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    // to share db ownership across threads
    // (if db moved in to one thread, db can't ne accessed by other threads),
    // we need ARC(Atomic reference counting) smart pointer (RC is restricted to single thread).
    let db = Arc::new(Mutex::new(Database::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let db = Arc::clone(&db);

        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            // calling connect method in db impl.
            db_lock.connect(i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Database connections: {:?}", db.lock().unwrap().connections);
}

// Send trait used to send its references across threads, if a type implements Send trait,
// it mean that developers are promising compiler that the type is safe,
// as the send trait is unsafe and can't be checked during compile time.

// RC smart pointer does not implement send, so we use Arc smart pointer.
// RC and RefCell does not implement Sync trait, mutex implements Sync trait,
// so we can use Arc<Mutex<T>> to share data across threads.


