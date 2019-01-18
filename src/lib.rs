extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
mod universe;
mod cell;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, welcome to wasm-game-of-life, {}", name));
}


#[wasm_bindgen]
pub fn univ() -> String {
    let univer = universe::Universe::new();
    //println!("{}", univer);
    univer.render()

}