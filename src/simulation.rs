use crate::structure;

pub const GRAVITY: f64 = -10.0;
pub const TIME_STEP: f64 = 1.0 / 60.0;
//pub const SIM_MIN_WIDTH: f64 = 20.0;

pub fn update(ball: &mut structure::Ball, window_width: &f64, window_height: &f64) {
    ball.vel_x += 0.0 * TIME_STEP; // no force from gravity hori
    ball.vel_y += GRAVITY * TIME_STEP;
    ball.pos_x += ball.vel_x * TIME_STEP;
    ball.pos_y += ball.vel_y * TIME_STEP;

    if ball.pos_x
        < ball.radius * 2.0 / structure::Ball::c_scale(&ball, window_width, window_height).unwrap()
    {
        ball.pos_x = ball.radius * 2.0
            / structure::Ball::c_scale(&ball, window_width, window_height).unwrap();
        ball.vel_x = ball.vel_x * -1.0;
    }

    if ball.pos_x
        > structure::Ball::sim_width(&ball, window_width, window_height).unwrap()
            - ball.radius * 2.0
                / structure::Ball::c_scale(&ball, window_width, window_height).unwrap()
    {
        ball.pos_x = structure::Ball::sim_width(&ball, window_width, window_height).unwrap()
            - ball.radius * 2.0
                / structure::Ball::c_scale(&ball, window_width, window_height).unwrap();
        ball.vel_x = ball.vel_x * -1.0;
    }

    if ball.pos_y
        < 0.0
            + ball.radius * 2.0
                / structure::Ball::c_scale(&ball, window_width, window_height).unwrap()
    {
        ball.pos_y = ball.radius * 2.0
            / structure::Ball::c_scale(&ball, window_width, window_height).unwrap();
        ball.vel_y = ball.vel_y * -1.0;
    }
}
