use distances::Vector3;

fn main() {
    let num_points = 60_000;
    let points = (0..num_points)
        .map(|i| {
            let i = i as f32;
            Vector3 { x: i, y: i, z: i }
        })
        .collect();
    let distances: Vec<_> = distances::get_distances(&points).collect();
    println!("{}", distances.len());
}
