import { BackEnd, wasm } from "./test.js";

const CELL_SIZE = 22; // px

let WindowHandle;
let Height, Width, Screen;
let Ctx;

function initDraw(data) {
    WindowHandle = BackEnd.new(data);
    Height = WindowHandle.height();
    Width = WindowHandle.width();
    const screenPtr = WindowHandle.screen();
    Screen = new Uint8Array(wasm.memory.buffer, screenPtr, Width * Height * 3);

    const canvas = document.getElementById("canvas");
    canvas.height = CELL_SIZE * Height;
    canvas.width = CELL_SIZE * Width;
    Ctx = canvas.getContext("2d");

    return;
}

const getIndex = (row, col, width) => {
    return row * width * 3 + col * 3;
}

const draw = () => {
    Ctx.beginPath();
    for (let row = 0; row < Height; row++) {
        for (let col = 0; col < Width; col++) {
            const idx = getIndex(row, col, Width);
            const r = Screen[idx];
            const g = Screen[idx + 1];
            const b = Screen[idx + 2];
            Ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
            Ctx.fillRect(
                col * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    Ctx.stroke();
}

const drawRun = () => {
    draw();
    const renderLoop = () => {
        WindowHandle.run()
        draw();
        setTimeout(() => {
            requestAnimationFrame(renderLoop);
        }, 20);
    }
    renderLoop();
}

export default initDraw;
export { draw, drawRun };