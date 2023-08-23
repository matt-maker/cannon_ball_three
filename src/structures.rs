pub const SIM_MIN_WIDTH: f64 = 20.0;

pub struct Ball {
    pub radius: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub vel_x: f64,
    pub vel_y: f64,
}

impl Ball {
    pub fn new(radius: f64, pos_x: f64, pos_y: f64, vel_x: f64, vel_y: f64) -> Self {
        Self {
            radius,
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        }
    }

    pub fn sim_width(&self, window_width: &f64, window_height: &f64) -> Option<f64> {
        Some(
            window_width
                / Ball::c_scale(&self, window_width, window_height)
                    .expect("c_scale not found in fn sim_width"),
        )
    }

    pub fn c_scale(&self, window_width: &f64, window_height: &f64) -> Option<f64> {
        Some(window_width.min(*window_height) / SIM_MIN_WIDTH)
    }

    pub fn cx(&self, window_width: &f64, window_height: &f64) -> Option<f64> {
        Some(
            self.pos_x
                * Ball::c_scale(&self, window_width, window_height)
                    .expect("c_scale not found in fn cx"),
        )
    }

    pub fn cy(&self, window_width: &f64, window_height: &f64) -> Option<f64> {
        Some(
            window_height
                - self.pos_y
                    * Ball::c_scale(&self, window_width, window_height)
                        .expect("c_scale not found in fn cy"),
        )
    }

    #[allow(dead_code)]
    pub fn sim_height(&self, window_width: &f64, window_height: &f64) -> Option<f64> {
        Some(
            window_height
                / Ball::c_scale(&self, window_width, window_height)
                    .expect("c_scale not found in fn sim_height"),
        )
    }
}
