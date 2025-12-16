// An attribute to hide warnings for unused code.
#![allow(dead_code)]


// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect : Rectangle) -> f32 {
    let Rectangle {top_left : Point {x: top_x, y : top_y}, bottom_right : Point {x: bottom_x, y: bottom_y}} = rect;
    (top_x - bottom_x)*(top_y - bottom_y).abs()
}

fn square(point : Point, wh : f32) -> Rectangle {
    let Point {x : x_coord, y :y_coord } = point;
    Rectangle {top_left : Point{x: x_coord, y : y_coord}, bottom_right : Point {x :x_coord + wh, y: y_coord + wh}}
}

fn main() {
    let point: Point = Point { x: 10.0, y: 5.0 };
    let another_point: Point = Point { x: 1.0, y: 1.0 };

    let rectangle = Rectangle {
        top_left: point,
        bottom_right: another_point,
    };
    let area = rect_area(rectangle);
    println!("{:?}", area);

    println!("{:?}", square(point, 5.5))
}