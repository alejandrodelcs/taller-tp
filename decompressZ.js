// decompressZ.js

export function decompressZ(inputBytes) {
    // Validar header .Z (0x1F 0x9D)
    if (inputBytes[0] !== 0x1F || inputBytes[1] !== 0x9D) {
        throw new Error("No es un archivo .Z válido");
    }

    const flags = inputBytes[2];
    const maxBits = flags & 0x1F; // normalmente 16
    const blockMode = (flags & 0x80) !== 0;

    let bitPos = 3 * 8; // empezamos después del header

    function readBits(n) {
        let result = 0;
        for (let i = 0; i < n; i++) {
            const bytePos = Math.floor(bitPos / 8);
            const bitOffset = bitPos % 8;

            const bit = (inputBytes[bytePos] >> bitOffset) & 1;
            result |= bit << i;

            bitPos++;
        }
        return result;
    }

    const CLEAR = 256;
    let codeSize = 9;
    let nextCode = 257;
    let maxCode = (1 << codeSize) - 1;

    // Diccionario inicial
    let dict = [];
    for (let i = 0; i < 256; i++) {
        dict[i] = [i];
    }

    let result = [];
    let prev = null;

    while (true) {
        if (bitPos + codeSize > inputBytes.length * 8) break;

        let code = readBits(codeSize);

        if (blockMode && code === CLEAR) {
            // reset
            dict = [];
            for (let i = 0; i < 256; i++) dict[i] = [i];

            codeSize = 9;
            nextCode = 257;
            maxCode = (1 << codeSize) - 1;
            prev = null;
            continue;
        }

        let entry;

        if (code < dict.length) {
            entry = dict[code];
        } else if (code === nextCode && prev !== null) {
            entry = prev.concat(prev[0]);
        } else {
            throw new Error("Código LZW inválido");
        }

        result.push(...entry);

        if (prev !== null) {
            dict[nextCode++] = prev.concat(entry[0]);
        }

        prev = entry;

        // aumentar tamaño de código
        if (nextCode > maxCode && codeSize < maxBits) {
            codeSize++;
            maxCode = (1 << codeSize) - 1;
        }
    }

    return new Uint8Array(result);
}