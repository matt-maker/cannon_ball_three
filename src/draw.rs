use crate::structure;
use std::f64;

pub fn render(
    window_width: &f64,
    window_height: &f64,
    ball: &structure::Ball,
    my_context: &web_sys::CanvasRenderingContext2d,
) {
    my_context.clear_rect(0.0, 0.0, *window_width, *window_height);
    my_context.begin_path();
    my_context
        .arc(
            structure::Ball::cx(&ball, &window_width, &window_height)
                .expect("cx value not found during render."),
            structure::Ball::cy(&ball, &window_width, &window_height)
                .expect("cy value not found during render"),
            ball.radius * 2.0,
            0.0,
            f64::consts::PI * 2.0,
        )
        .expect("failed to draw arc");
    my_context.fill();
}
