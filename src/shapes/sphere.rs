use nalgebra::{distance, Point3};
use Container;

#[derive(PartialEq, Debug, Clone)]
/// Constructs a sphere located at `center` in Euclidean space with a given `radius`.
pub struct Sphere {
    /// Central point in space where this sphere is located.
    pub center: Point3<f32>,
    /// Radius of current sphere.
    pub radius: f32,
}

impl Sphere {
    /// Creates a `new` sphere given the location of the spheres' `center` and its' `radius`.
    pub fn new(center: Point3<f32>, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }

    /// If the distance between the centers of two spheres is less than the sum of
    /// their radii, we can consider them to be overlapping. Will return `true` in this case.
    pub fn overlaps(&self, other: &Sphere) -> bool {
        distance(&self.center, &other.center) < self.radius + other.radius
    }
}

impl Container for Sphere {
    /// Checks if sphere exists inside the current bounding sphere.
    fn contains(&self, sphere: &Sphere) -> bool {
        distance(&Point3::origin(), &sphere.center) + sphere.radius < self.radius
    }
}