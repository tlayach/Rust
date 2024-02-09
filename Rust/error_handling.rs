fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        println!("Going for the error.");
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result: Result<i32, String> = divide(10, 0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
