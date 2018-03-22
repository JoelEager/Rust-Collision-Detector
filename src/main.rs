mod sat;

use sat::Point;

fn main() {
    let tank = vec![Point(195, 95), Point(205, 95), Point(205, 105), Point(195, 105)];
    let shell = vec![Point(199.5, 99.5), Point(200.5, 99.5), Point(200.5, 100.5), Point(199.5, 100.5)];
    println!("Result: {}", sat::has_collided(poly1, poly2, None));
}