import { Universe, Cell } from "wasm-game-of-life";	        var start = new Date().getTime();
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";	        while (new Date().getTime() < start + delay);

       }
const CELL_SIZE = 5; // px	 //alert('swapnil');
const GRID_COLOR = "#CCCCCC";	var element = document.querySelector('body');
const DEAD_COLOR = "#FFFFFF";	//console.log(element.innerHTML);
const ALIVE_COLOR = "#000000";	var univ = wasm.get_universe();

 function game_loop(){
// Construct the universe, and get its width and height.	     element.innerText = univ.render();
const universe = Universe.new();	    univ.tick();
const width = universe.width();	    sleep(1000);
const height = universe.height();	    debugger;

     //console.log(univ.total_living);
// Give the canvas room for all of our cells and a 1px border	    requestAnimationFrame(game_loop);
// around each of them.	 }
const canvas = document.getElementById("game-of-life-canvas");	 game_loop(); 
canvas.height = (CELL_SIZE + 1) * height + 1;	
canvas.width = (CELL_SIZE + 1) * width + 1;	

 const ctx = canvas.getContext('2d');	

 const getIndex = (row, column) => {	
  return row * width + column;	
};	

 const drawGrid = () => {	
	ctx.beginPath();	
	ctx.strokeStyle = GRID_COLOR;	

 	//Vertical lines.	
	for ( let i = 0; i <= width; i++){	
		ctx.moveTo(i* ( CELL_SIZE+1 ) + 1, 0);	
		ctx.lineTo(i* ( CELL_SIZE+1 ) + 1, height* ( CELL_SIZE+1 ) + 1);	}	

 	//Horizontal lines.	
	for (let j = 0; j <= height; j++){	
		ctx.moveTo(0, 				j* ( CELL_SIZE+1 ) + 1);	
		ctx.lineTo( ( CELL_SIZE+1 )* width + 1, j* ( CELL_SIZE+1 ) + 1);	
	}	

 	ctx.stroke();	
};	

 const drawCells = () => {	
	const cellsPtr = universe.cells();	
	const cells = new Uint8Array( memory.buffer, cellsPtr, width*height);	
	ctx.beginPath();	
	for (let row = 0; row < height; row++){	
		for (let col = 0; col < width; col++){	
			const idx = getIndex(row, col);	
			ctx.fillStyle = cells[idx] === Cell.Dead	
				? DEAD_COLOR	
				: ALIVE_COLOR;	
			ctx.fillRect(	
				col * (CELL_SIZE + 1) + 1,	
				row * (CELL_SIZE + 1) + 1,	
				CELL_SIZE,	
				CELL_SIZE	
			);	
		}	
	}	
	ctx.stroke();	
};	




 const renderLoop = () => {	
  //debugger;	
  universe.tick();	

   drawGrid();	
  drawCells();	

   requestAnimationFrame(renderLoop);	
};	
renderLoop(); 