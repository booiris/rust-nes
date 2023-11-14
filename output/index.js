import init from "./rust_nes.js";
import initDraw, { drawRun } from "./draw.js";

const run = async () => {
    await initDraw()
    drawRun()
}

await init();
run();