type RGB = (f64, f64, f64);

pub const SIM_TIME_STEP: f64 = 2.0;
pub const SIM_TIME_DURATION: f64 = 200.0;
pub const SIM_FRAME_RATE: i32 = 5;

pub const ACTOR_COLOR: RGB = (0.1, 0.7, 0.1);
pub const ACTOR_RADIUS_VISUAL: f64 = 0.01;
pub const ACTOR_MAX_SPEED: f64 = 0.02;

pub const ROAD_JUNCTION_COLOR: RGB = (0.7, 0.7, 0.7);
pub const ROAD_JUNCTION_RADIUS: f64 = 0.05;

pub const ROAD_LANE_ARROW_SIZE: f64 = 0.01;
pub const ROAD_LANE_COLOR: RGB = (0.7, 0.3, 0.7);
pub const ROAD_LANE_WIDTH: f64 = 0.025;
pub const ROAD_LANE_WIDTH_VISUAL: f64 = 0.005;

pub const ROAD_SEGMENT_COLOR: RGB = (1.0, 1.0, 1.0);
pub const ROAD_SEGMENT_WIGGLE_ROOM_PCT: u32 = 20;

pub const FILLED_SHAPE_BORDER_WIDTH: f64 = 0.001;
