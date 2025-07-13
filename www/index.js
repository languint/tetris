import { Game } from "wasm-tetris";

const sleep = ms => new Promise(r => setTimeout(r, ms));

async function run() {
  const wasmModule = await import("wasm-tetris");

  if (typeof wasmModule.init === 'function') {
    await wasmModule.init();
  }


  const game = new Game();

  window.onresize = () => game.resize();

  let lastTime = 0;
  const renderLoop = (currentTime) => {
    const deltaTime = currentTime - lastTime;
    lastTime = currentTime;

    game.tick(deltaTime);

    requestAnimationFrame(renderLoop);
  };

  const normalButton = document.querySelector("#normal-button");

  normalButton.addEventListener("click", async (e) => {
    game.hide_menu();
    await sleep(500);
    game.show_game();
    await sleep(500);
    game.resume_game();
  })

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
        break;
      case "KeyS":
      case "ArrowDown":
        game.soft_drop();
        break;
    }
  });

  requestAnimationFrame(renderLoop);
}

run().catch(e => console.error("Error in run() function:", e));
