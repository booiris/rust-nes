import { read_file } from "./foo.js";
import init from "./test.js";
import initDraw, { drawRun } from "./draw.js";

const run = (data) => {
    initDraw(data)
    drawRun()
}

await init();
let data = await read_file("./snake.nes");
run(data);