pub struct Vector(pub f64, pub f64);

pub fn has_collided(poly1: Vec<Vector>, poly2: Vec<Vector>, max_dist: Option<i32>) -> bool {
    // Checks for a collision between two convex 2D polygons using separating axis theorem (SAT)
    return true;
}

fn edge_vector(point1: &Vector, point2: &Vector) -> Vector {
    // Returns a vector going from point1 to point2
    return Vector(point2.0 - point1.0, point2.1 - point1.1);
}

fn poly_to_edges(poly: Vec<Vector>) -> Vec<Vector> {
    // Returns a Vec of the edges of the poly as vectors
    let mut edges = Vec::new();

    for index in 0..poly.len() {
        edges.push(edge_vector(&poly[index], &poly[index % poly.len()]));
    }

    return edges;
}

fn orthogonal(vector: &Vector) -> Vector {
    // Returns a new vector which is orthogonal to the given vector
    return Vector(vector.1, -vector.0);
}

fn dot_product(vector1: &Vector, vector2: &Vector) -> f64 {
    // Returns the dot (or scalar) product of the two vectors
    return vector1.0 * vector2.0 + vector1.1 * vector2.1;
}