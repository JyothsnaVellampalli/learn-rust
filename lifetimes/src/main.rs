fn main() {
    let val1 = String::from("value1");
    let val2 = String::from("value2");

    // The lifetime of res is smallest of p1, p2
    let res = check(val1.as_str(), val2.as_str());

    println!("value: {}", res);
}

fn check<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

// Notes:
// static lifetime - lives for the entire duration of the prog.
// &str = &'static str