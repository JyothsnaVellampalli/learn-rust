use tokio_stream::StreamExt;

// a future can return only one value,
// whereas a stream can return series of asynchronous values.

#[tokio::main]
async fn main() {
    let mut stream =
        tokio_stream::iter(["abc", "def", "ghi"])
        .map(|x| { x.to_uppercase() });

    // next() returns a Future.
    while let Some(value) = stream.next().await {
        println!("{value}");
    }
}

