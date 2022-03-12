#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(rust_2018_idioms)]
#![warn(clippy::all)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = evilset::EvilSetApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
