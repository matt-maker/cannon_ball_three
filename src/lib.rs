use crate::JsValue;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod dev_support;
mod draw;
mod setup;
mod simulation;
mod structures;
mod web_support;

#[allow(unused)]
use dev_support as support;
use draw as render;
use structures as structure;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let my_window: web_sys::Window = web_support::window();
    let my_canvas: web_sys::HtmlCanvasElement = web_support::canvas("canvas");
    let my_context: web_sys::CanvasRenderingContext2d = web_support::context(&my_canvas);
    let window_height: f64 = my_window.inner_height().unwrap().as_f64().unwrap() - 20.0;
    let window_width: f64 = my_window.inner_width().unwrap().as_f64().unwrap() - 20.0;

    let mut ball: structure::Ball = structure::Ball::new(15.0, 50.0, 10.0, 3.0, 15.0);

    #[allow(unused_variables, unused_mut)]
    let performance = my_window.performance().expect("issue getting ms time log");
    //let mut duration_time = String::new();
    //let mut end_time: f64 = 0.0;

    setup::setup(&my_context, &my_canvas, &window_width, &window_height);

    *g.borrow_mut() = Some(Closure::new(move || {
        //duration_time = support::get_duration(&mut end_time, &performance).to_string();
        //support::console_out("time log in ms", &duration_time);

        web_support::request_animation_frame(f.borrow().as_ref().unwrap());
        simulation::update(&mut ball, &window_width, &window_height);
        render::render(&window_width, &window_height, &ball, &my_context);
    }));

    web_support::request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
