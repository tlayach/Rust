fn area(x: i32, y: i32) -> i32 {
    return x * y;
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    return x * y * z;
}

fn main() {
    // constant
    let width: i32 = 4;
    let height: i32 = 10;
    let depth: i32 = 5;

    // area
    let area: i32 = area(width, height);
    println!("The area, for width {width} and height {height} is {area}.");

    // volume
    let volume: i32 = volume(width, height, depth);
    println!("The volume, for width {width}, height {height} and depth {depth} is {volume}.");
}
