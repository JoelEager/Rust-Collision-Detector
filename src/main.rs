mod sat;

use sat::Vector;

fn main() {
    let tank = vec![Vector(195.0, 95.0), Vector(205.0, 95.0), Vector(205.0, 105.0),
                    Vector(195.0, 105.0)];
    let shell = vec![Vector(199.5, 99.5), Vector(200.5, 99.5), Vector(200.5, 100.5),
                     Vector(199.5, 100.5)];

    println!("Result: {}", sat::has_collided(tank, shell, None));
}