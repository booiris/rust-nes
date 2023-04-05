import init, { Window, wasm } from "./test.js";
import { read_file } from "./foo.js";

const CELL_SIZE = 22; // px

const run = (data) => {
    const window = Window.new(data);
    const height = window.height();
    const width = window.width();

    const canvas = document.getElementById("canvas");
    canvas.height = CELL_SIZE * height;
    canvas.width = CELL_SIZE * width;
    const ctx = canvas.getContext("2d");

    let iter = 0;
    draw(window, ctx);
    const renderLoop = () => {
        if (window.tick()) {
            console.log("drawing");
            draw(window, ctx);
        }
        iter += 1;
        // if (iter > 1000) {
        //     return
        // }
        // console.log(iter);
        setTimeout(() => requestAnimationFrame(renderLoop), 0);
    }
    renderLoop();

}

const getIndex = (row, col, width) => {
    return row * width * 3 + col * 3;
}

const draw = (window, ctx) => {
    const height = window.height();
    const width = window.width();
    const screenPtr = window.screen();
    const screen = new Uint8Array(wasm.memory.buffer, screenPtr, width * height * 3);

    ctx.beginPath();
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col, width);
            const r = screen[idx];
            const g = screen[idx + 1];
            const b = screen[idx + 2];
            ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
            ctx.fillRect(
                col * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
}

await init();
let data = await read_file("./snake.nes");
run(data);