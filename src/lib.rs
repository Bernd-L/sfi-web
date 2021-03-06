#![recursion_limit = "512"]

pub(crate) mod components;
pub(crate) mod constants;
pub(crate) mod types;

use components::app;
use wasm_bindgen::prelude::*;
use yew::web_sys::console;

/// The app's main entry point
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Log the license notice
    console::group_1(&constants::license::license_notice_title().into());
    console::log_1(&constants::license::license_notice_body().into());
    console::group_end();

    // Start the yew app
    yew::start_app::<app::App>();

    // Clean exit
    Ok(())
}
