use log::info;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Initialize panic logging
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Initialize regular logging
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    info!("Inside Rust start() function");

    match web_sys::window() {
        Some(window) => info!("window in worker context is: {:?}", window),
        None => info!("window is None in worker context"),
    }
    Ok(())
}
