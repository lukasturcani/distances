use num_traits::Float;

pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub fn get_distances<T: Float>(points: &Vec<Vector3<T>>) -> impl Iterator<Item = T> + '_ {
    points.iter().enumerate().flat_map(|(i, point1)| {
        points
            .iter()
            .skip(i + 1)
            .map(|point2| distance(point1, point2))
    })
}

fn distance<T: Float>(point1: &Vector3<T>, point2: &Vector3<T>) -> T {
    (squared_difference(point1.x, point2.x)
        + squared_difference(point1.y, point2.y)
        + squared_difference(point1.z, point2.z))
    .sqrt()
}

fn squared_difference<T: Float>(a: T, b: T) -> T {
    let difference = a - b;
    difference * difference
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distances() {
        let points = vec![
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Vector3 {
                x: 1.,
                y: 0.,
                z: 0.,
            },
        ];
        let distances: Vec<f32> = get_distances(&points).collect();
        assert_eq!(distances, vec![1.0]);
    }
}
