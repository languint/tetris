
import { Game } from "wasm-tetris";

const game = new Game();

window.onresize = () => game.resize();

const renderLoop = () => {
  game.tick();
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
