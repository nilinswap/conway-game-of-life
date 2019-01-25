import * as wasm from "wasm-game-of-life";
 //wasm.univ();
function sleep(delay) {
        var start = new Date().getTime();
        while (new Date().getTime() < start + delay);
      }
 //alert('swapnil');
var element = document.querySelector('body');
//console.log(element.innerHTML);
var univ = wasm.get_universe();
function game_loop(){
     element.innerText = univ.render();
    univ.tick();
    sleep(1000);
    debugger;
    //console.log(univ.total_living);
    requestAnimationFrame(game_loop);
 }
 game_loop();