
import { Board } from "wasm-tetris";
import * as wasm from "wasm-tetris";

const board = new Board();

window.onresize = () => {console.log("A"); wasm.resize(board)};

wasm.resize(board);

wasm.run();
