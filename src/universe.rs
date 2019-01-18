use crate::cell;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
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
		let mut count = 0;
		for del_row in [self.height - 1, 0 , 1].iter().cloned(){
			for del_col in [self.width - 1, 0, 1].iter().cloned(){
				if del_row == 0 && del_col == 0{
					continue;
				}
				let nei_row = (row + del_row) % self.height;
				let nei_col = (column + del_col) % self.width;
				let idx = self.get_index(nei_row, nei_col);
				count += self.cells[idx] as u8
			}
		}
		count
	}
}

#[wasm_bindgen]
impl Universe{
	pub fn new() -> Self{

		let width :u32 = 16;
		let height:u32 = 16;
		let mut cells :Vec<cell::Cell> = vec![];
		for i in 0..width*height{
			if i%2 == 0 || i%7==0{
				cells.push(cell::Cell::Alive);
			}
			else{
				cells.push(cell::Cell::Dead);
			}
		}
		//let cells: Vec<cell::Cell> = vec![cell::Cell::dead;(width*height) as usize];
		Universe{
			width ,
			height,
			cells,

		}
	}
	pub fn tick(&mut self){
		unimplemented!();
	}
	pub fn render(&self) -> String{
		self.to_string()
	}
}

use std::fmt;
impl fmt::Display for Universe{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		let mut st = String::new();

		for row in 0..self.height{
			for col in 0..self.width{
				match self.cells[self.get_index(row, col)]{
					cell::Cell::Alive => st += "◼",
					cell::Cell::Dead  => st += "◻",
				}

			}
			st += "\n";

		}
		write!(f, "{}", &st)
	}


}



