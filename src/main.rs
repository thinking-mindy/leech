
#![deny(unsafe_code)]
slint::include_modules!();

//add booty bin
//decoy
//
//
//
//
//
//
//
//

#[tokio::main]
async fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let app = App::new().unwrap();
    

    app.run().unwrap();
}
