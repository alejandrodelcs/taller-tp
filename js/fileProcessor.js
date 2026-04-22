import { decompressZ } from "./decompressZ.js";

export function procesarArchivo(bytes) {

    // .Z (LZW)
    if (bytes[0] === 0x1F && bytes[1] === 0x9D) {
        console.log("→ .Z detectado");
        return decompressZ(bytes);
    }

    // .gz (gzip)
    if (bytes[0] === 0x1F && bytes[1] === 0x8B) {
        console.log("→ .gz detectado");
        return fflate.gunzipSync(bytes);
    }

    console.log("→ sin compresión");
    return bytes;
}