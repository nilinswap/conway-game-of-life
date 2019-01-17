mod cell;

#[wasm_bindgen]
pub struct Universe{
	width: u32,
	height: u32,
	cells: Vec<cell::Cell>,
}
impl Universe{
	fn get_index(&self, row: u32, column: u32) -> usize{
		(row*self.width + column) as usize
	}
	fn  live_neighbour_count(&self, row: u32, column: u32) -> u8{
		
	}
}

#[wasm_bindgen]
impl Universe{
	pub new() -> self{

	}
	pub fn tick(&mut self){
	}
	pub fn render(&self) -> String{
		self.to_string()
	}
}

use std::fmt;
impl fmt::Display for Universe{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		
	}
}



