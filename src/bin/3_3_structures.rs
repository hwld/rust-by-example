#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn rect_area(rec: &Rectangle) -> f32 {
  let Rectangle {
    top_left : Point {x: top_left_x, y: top_left_y},
    bottom_right: Point {x: bottom_right_x, y: bottom_right_y}
  } = rec;

  (bottom_right_x - top_left_x) * (top_left_y - bottom_right_y)
}

fn square(point: &Point, side: f32) -> Rectangle {
  Rectangle {
    top_left: Point { x: point.x, y: point.y + side  },
    bottom_right: Point { x: point.x + side , y: point.y },
  }
}

fn main() {
  let rect = Rectangle { 
    top_left:  Point {x: 0.0, y: 5.0},
    bottom_right: Point {x: 5.0, y: 0.0},
  };
  println!("{:?}'s area is {}", rect, rect_area(&rect));

  let square = square(&Point {x: 0.0, y: 0.0}, 5.0);
  println!("{:?}'s area is {}", square, rect_area(&square));
}