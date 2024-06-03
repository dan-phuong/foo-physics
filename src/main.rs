// This module is used for calculating the volume of a sphere 
// but can be expanded upon for other shapes. 
// Sphere and other 

use std::f64::consts::PI;
use num::{Float, FromPrimitive};

/// Sphere struct for a pattern similar to Object Oriented and composition
struct Sphere {
    radius: f64
}

/// Volume trait for shapes.
trait Volume {
    fn volume(&self) -> f64;
}

impl Volume for Sphere {
    /// Implementation of the volume of a sphere.
    fn volume(&self) -> f64 {
        4.0 / 3.0 * PI * self.radius.powi(3)
    }
}

/// Generic function to calculate volume of a sphere.
/// Restricted to only float32 and float64 primitive types
pub fn sphere_volume<T: Float + FromPrimitive>(radius: T) -> T{
    radius.powi(3) * T::from_f64(4.0/3.0).unwrap() * T::from_f64(PI).unwrap() //radius.powi(3);
}


/// Example of adding another shape using similar pattern to OOP
/// using a Cylinder struct and volume trait
struct Cylinder {
    radius: f64,
    height: f64,
}

impl Volume for Cylinder {
    fn volume(&self) -> f64 {
        PI * self.height * self.radius.powi(2)
    }
}

#[test]
pub fn test_sphere_volume_struct(){
    let sphere = Sphere {
        radius: 5.0 
    };
    assert_eq!(sphere.volume(), 523.5987755982989);
    assert_ne!(sphere.volume(), 14.0);
}

#[test]
pub fn test_sphere_volume(){
    assert_eq!(sphere_volume(5.0), 523.5987755982989);
    assert_ne!(sphere_volume(3.0), 523.5987755982989);

} 
fn main() {
    println!("Hello, world!");
}
