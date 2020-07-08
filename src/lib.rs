
#![allow(unused_variables)]
fn main() {
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            let val = document.create_element("h1")?;
            val.set_inner_html("hey 3!");
        
            body.append_child(&val)?;
            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);
        let val = document.create_element("h1")?;
        val.set_inner_html("Rust!!");
    
        body.append_child(&val)?;
        if count == 10 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
    Ok(())
}

}