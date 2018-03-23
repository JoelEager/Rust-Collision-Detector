// A Rust implementation of separating axis theorem

#[derive(Clone, Copy)]
pub struct Vector(pub f32, pub f32);

pub fn has_collided(poly1: &[Vector], poly2: &[Vector], max_dist: &Option<f32>) -> bool {
    /* Checks for a collision between two convex 2D polygons using separating axis theorem (SAT)
    poly1, poly2: The two polygons described as lists of points as tuples
        Example: [(x1, y1), (x2, y2), (x3, y3)]
        Note: The points list must go in sequence around the polygon
    maxDist: The maximum distance between any two points of any two polygons that can be touching
        If this None then the optimization check that uses it will be skipped
    */
    let estimated_dist = (poly1[1].0 - poly2[0].0).powi(2) + (poly1[1].1 - poly2[0].1).powi(2);
    return match max_dist {
        &Some(max_dist) if estimated_dist > max_dist.powi(2) => false,
        &Some(_) | &None => run_sat(poly1, poly2)
    };
}

fn run_sat(poly1: &[Vector], poly2: &[Vector]) -> bool {
    // Implements the actual SAT algorithm
    let mut edges = Vec::new();
    edges.append(&mut poly_to_edges(&poly1));
    edges.append(&mut poly_to_edges(&poly2));

    let axes = edges.into_iter().map(orthogonal);

    for axis in axes {
        if !overlap(project(&poly1, axis), project(&poly2, axis)) {
            // The polys don't overlap on this axis so they can't be touching
            return false;
        }
    }

    // The polys overlap on all axes so they must be touching
    return true;
}

fn edge_vector(point1: Vector, point2: Vector) -> Vector {
    // Returns a vector going from point1 to point2
    return Vector(point2.0 - point1.0, point2.1 - point1.1);
}

fn poly_to_edges(poly: &[Vector]) -> Vec<Vector> {
    // Returns a Vec of the edges of the poly as vectors
    let mut edges = Vec::with_capacity(poly.len());

    for index in 0..poly.len() {
        edges.push(edge_vector(poly[index], poly[(index + 1) % poly.len()]));
    }

    return edges;
}

fn orthogonal(vector: Vector) -> Vector {
    // Returns a new vector which is orthogonal to the given vector
    return Vector(vector.1, -vector.0);
}

fn dot_product(vector1: Vector, vector2: Vector) -> f32 {
    // Returns the dot (or scalar) product of the two vectors
    return vector1.0 * vector2.0 + vector1.1 * vector2.1;
}

fn project(poly: &[Vector], axis: Vector) -> Vector {
    // Returns a vector showing how much of the poly lies along the axis
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;

    for point in poly.iter() {
        let dot = dot_product(*point, axis);

        match min {
            Some(val) if val < dot => (),
            _ => min = Some(dot)
        }
        match max {
            Some(val) if val > dot => (),
            _ => max = Some(dot)
        }
    }

    return Vector(min.unwrap(), max.unwrap());
}

fn overlap(projection1: Vector, projection2: Vector) -> bool {
    // Returns a boolean indicating if the two projections overlap
    return projection1.0 <= projection2.1 && projection2.0 <= projection1.1
}