use std::ops;

extern crate meaningless_app;
use meaningless_app::a_plus_b;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


fn main() {
    println!("Hello, world!");
    let a = Point{ x: 0, y: 100 };
    let b = Point{ x: 234, y: 1 };
    let a2 = a.clone();
    let b2 = b.clone();
    println!("a + b: {:?}", a + b);
    println!("a_plus_b(a, b): {:?}", a_plus_b(a2, b2));
}
