
import { Game } from "wasm-tetris";

const game = new Game();

window.onresize = () => game.resize();

let lastTime = 0;
const renderLoop = (currentTime) => {
  const deltaTime = currentTime - lastTime;
  lastTime = currentTime;

  game.tick(deltaTime);

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
