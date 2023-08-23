pub fn setup(
    my_context: &web_sys::CanvasRenderingContext2d,
    my_canvas: &web_sys::HtmlCanvasElement,
    window_width: &f64,
    window_height: &f64,
) {
    my_canvas.set_width(*window_width as u32);
    my_canvas.set_height(*window_height as u32);
    my_context.set_fill_style(&wasm_bindgen::JsValue::from_str("blue"));
}
