use super::super::{ColorName, KdNode, Point, Rgb, coord};

fn euclidean_distance(a: &Point, b: &Point) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn nearest(
    node: &Option<Box<KdNode>>,
    target: &Point,
    depth: usize,
    best: Option<(ColorName, f32)>,
) -> Option<(ColorName, f32)> {
    let node = match node {
        Some(n) => n,
        None => return best,
    };

    let axis = depth % 3;

    let node_point = node.color.point;
    let d = euclidean_distance(target, &node_point);

    let mut best = match best {
        Some((c, bd)) if bd < d => Some((c, bd)),
        _ => Some((node.color.clone(), d)),
    };

    let diff = coord(target, axis) - coord(&node_point, axis);

    let (first, second) = if diff < 0.0 {
        (&node.left, &node.right)
    } else {
        (&node.right, &node.left)
    };

    best = nearest(first, target, depth + 1, best);

    let best_dist = best.clone().map(|(_, d)| d).unwrap_or(f32::INFINITY);

    if diff * diff < best_dist {
        best = nearest(second, target, depth + 1, best);
    }

    best
}

pub fn get_color_name_match(color: &Rgb, tree: Option<Box<KdNode>>) -> String {
    let target = Point {
        x: color.r as f32 / 255.0,
        y: color.g as f32 / 255.0,
        z: color.b as f32 / 255.0,
    };
    match nearest(&tree, &target, 0, None) {
        Some((c, _)) => c.to_owned().name,
        None => "Unknown".to_string(),
    }
}
