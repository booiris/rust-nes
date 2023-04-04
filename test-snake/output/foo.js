// foo.js

export async function read_file(path) {
    let resp = await fetch(path);
    let buffer = await resp.arrayBuffer();
    const data = new Uint8Array(buffer);
    return data;
}