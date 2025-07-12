
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

addEventListener("keydown", (e) => {
  switch (e.code) {
    case "KeyR":
    case "ArrowUp":
      game.rotate_current_piece();
      break;
    case "KeyA":
    case "ArrowLeft":
      game.move_cursor_left();
      break;
    case "KeyD":
    case "ArrowRight":
      game.move_cursor_right();
      break;
    case "Space":
      game.hard_drop_current_piece();
      break;
    case "KeyC":
    case "ShiftLeft":
    case "ShiftRight":
      game.hold_piece();
    case "KeyS":
    case "ArrowDown":
      game.soft_drop();
  }
})

requestAnimationFrame(renderLoop);
