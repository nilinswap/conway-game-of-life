extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell{
	Dead = 0,
	Alive = 1,
}

