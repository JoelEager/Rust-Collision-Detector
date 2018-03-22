mod sat;

use sat::Point;

fn main() {
    let poly1 = vec![Point(10, 10), Point(20, 10)];
    let poly2 = vec![Point(10, 10), Point(20, 10)];
    println!("Result: {}", sat::has_collided(poly1, poly2, None));
}