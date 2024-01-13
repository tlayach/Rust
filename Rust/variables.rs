fn main() {
    // constant
    const EARTH: i32 = 10;

    // mutable
    let mut speed = 123;

    // immutable
    let ready: i32 = 2;

    println!("At {EARTH}, go at speed {speed} and get ready on {ready}.");

    speed = speed * ready;
    println!("New speed of {speed}");
}
