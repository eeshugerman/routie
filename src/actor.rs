extern crate nalgebra;

use nalgebra::Point2;

pub struct Actor {
    id: i32,
    location: Point2<f64>,
    max_speed: f64,
    // route: Option<Vec<RouteStep>>,
    // agenda: Vec<AgendaStep>
}
