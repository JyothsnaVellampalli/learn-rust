use tokio::time::sleep;
use std::time::Duration;

// aync code follows cooperative scheduling model.
// task = lighweight, non-blocking unit of execution
// tasks allow top level futures to run concurrently

async fn my_function(i: i32) {
    println!("[{i}] I am an async function!");
    let s1 = read_from_db().await;
    println!("[{i}] first: , {}", s1);
    let s2 = read_from_db().await;
    println!("[{i}] second: , {}", s2);
}

async fn read_from_db() -> String {
    // to simulate db operation.
    // sleep function will delay the execution of the current future for a given duration
    // instead of blocking the entire thread.
    sleep(Duration::from_secs(1)).await;
    String::from("hello from DB")
}

// aync functions are not suitable for CPU-intensive tasks,
// as it may block all other async tasks in that thread.
fn expensive_computation() {
    let mut i = 0;
    for _ in 0..1000000000 {
        i = i + 1;
    }
    println!("expensive computation done! {i}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];

    // By default, tokio uses a thread pool to execute tasks in parllel.
    // By adding flavour #[tokio::main(flavor = "current_thread")], 
    // we can restrict to use only one thread, where tasks run concurrently using time slicing.
    for i in 0..2 {
        // move is mentioned to allow 
        // async block take ownership of the variables in this block.
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    handles.push(tokio::spawn(async {
        // this will block the other tasks running in the same thread.
        // expensive_computation();

        // to avoid blocking of other tasks running in the same thread.
        let _ = tokio::task::spawn_blocking(|| {
            // this will run in a separate thread pool where blocking is acceptable.
            expensive_computation();
        }).await;
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}