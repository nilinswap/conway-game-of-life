use crate::cell;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
extern crate web_sys;
use utils;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
pub struct Universe{
	width: u32,
	height: u32,
	cells: Vec<cell::Cell>,
	total_living: u32,
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, welcome to wasm-game-of-life, {}", name));
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

		utils::set_panic_hook();


		let width :u32 = 64;
		let height:u32 = 64;
		let mut cells :Vec<cell::Cell> = vec![];
		let mut total_living = 0;
		for i in 0..width*height{
			if i%7 == 0 || i%3 ==0{
				cells.push(cell::Cell::Alive);
				total_living += 1;
			}
			else{
				cells.push(cell::Cell::Dead);
			}

		}
		//let cells: Vec<cell::Cell> = vec![cell::Cell::dead;(width*height) as usize];
		Universe{
			width,
			height,
			cells,
			total_living,


		}
	}

	pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn tick(&mut self){
		let mut cells= self.cells.clone();
		for row in 0..self.height{
			for col in 0..self.width{
				let idx = self.get_index( row, col );
				let cell = cells[idx];
				let nei_count = self.live_neighbour_count(row, col);
				 log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    nei_count
				 );
				let next_cell = match (nei_count, self.cells[idx]){
												(x, cell::Cell::Alive)if x < 2 || x > 3  =>{ self
													.total_living -=1;
													cell::Cell::Dead},
												(x, cell::Cell::Alive)if x == 2 || x == 3 =>
													{
													cell::Cell::Alive},
												(3, cell::Cell::Dead) => { self
													.total_living +=1;
													cell::Cell::Alive},
												(_, _)	=> {
													cell::Cell::Dead},


											};
				cells[idx] = next_cell;
				//let a = next_cell;

				log!("    it becomes {:?}", next_cell);
			}
		}
		self.cells = cells;
		//println!("{:?}", cells[0]);
		//alert(&self.total_living.to_string());

	}
	pub fn render(&self) -> String{
		self.to_string()
	}

	pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const cell::Cell {
        self.cells.as_ptr()
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
		write!(f, "{}", &st)?;

		Ok(())
	}



}

impl Universe{

	pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| cell::Cell::Dead).collect();
    }

	pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| cell::Cell::Dead).collect();
    }

	pub fn get_cells(&self) -> &[cell::Cell] {
        &self.cells
    }

	pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = cell::Cell::Alive;
        }
    }

}



