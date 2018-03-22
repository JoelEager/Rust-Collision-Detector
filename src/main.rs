mod sat;

use sat::Vector;

fn main() {
    let tank = vec![Vector(195.0, 95.0), Vector(205.0, 95.0), Vector(205.0, 105.0),
                    Vector(195.0, 105.0)];
    let shell1 = vec![Vector(199.5, 99.5), Vector(200.5, 99.5), Vector(200.5, 100.5),
                     Vector(199.5, 100.5)];
    let shell2 = vec![Vector(299.5, 199.5), Vector(300.5, 199.5), Vector(300.5, 200.5),
                     Vector(299.5, 200.5)];

    println!("Result 1: {}", sat::has_collided(&tank, &shell1, None));
    println!("Result 2: {}", sat::has_collided(&tank, &shell2, None));
}