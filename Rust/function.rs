fn area(x: i32, y: i32) -> i32 {
    return x * y;
}

fn main() {
    // constant
    let width: i32 = 4;
    let height: i32 = 10;

    let area: i32 = area(width, height);

    println!("The area, for width {width} and height {height} is {area}.");
}
