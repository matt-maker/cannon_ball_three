use crate::JsValue;
use web_sys::console;

#[allow(dead_code)]
pub fn console_out(message1: &str, message2: &str) {
    let message1: JsValue = message1.into();
    let message2: JsValue = message2.into();
    console::log_4(&"log of: ".into(), &message1, &" : ".into(), &message2);
}

#[allow(dead_code)]
pub fn get_duration(end_time: &mut f64, performance: &web_sys::Performance) -> f64 {
    let start_time = performance.now();
    let duration = start_time - *end_time;
    *end_time = start_time;
    duration
}
