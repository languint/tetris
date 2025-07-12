import { Board } from "wasm-tetris";
import * as wasm from "wasm-tetris";

wasm.run();

const board = new Board();
const boardWidth = board.width();
const boardHeight = board.height();

const gameContainer = document.querySelector(".game-container");

function sizeElements() {
    const availableWidth = gameContainer.clientWidth;
    const availableHeight = gameContainer.clientHeight;

    const cellSize = Math.min(
        availableWidth / boardWidth,
        availableHeight / boardHeight
    );

    const canvas = document.querySelector(".game-canvas");
    canvas.width = boardWidth * cellSize;
    canvas.height = boardHeight * cellSize;

    document.documentElement.style.setProperty("--cell-size", `${cellSize}px`);
}


window.onresize = sizeElements;
sizeElements();