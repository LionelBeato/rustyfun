#[derive(Debug)] // need to understand this line

// classic struct, something of a class
struct Person {
    name: String,
    age: u8,
}

struct Point {
    x: f32,
    y: f32,
}

// unit struct, good for generics
struct Unit;

// tuple struct, otherwise named custom tuples
struct Pair(i32, f32);

#[allow(dead_code)] // decorator or annotation
struct Rectangle {
    top_left: Point,
    bottom_right: Pont,
}
