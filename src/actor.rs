extern crate nalgebra;

use nalgebra::Point2;

#[allow(dead_code)]
pub struct Actor {
    id: i32,
    location: Point2<f64>,
    max_speed: f64,
    // route: Option<Vec<RouteStep>>,
    // agenda: Vec<AgendaStep>
}
