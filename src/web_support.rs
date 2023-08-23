use wasm_bindgen::prelude::*;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global 'window' exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no document on window")
}

pub fn canvas(element_name: &str) -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id(element_name)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'request_animation_frame' ok");
}

pub fn context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context
}
