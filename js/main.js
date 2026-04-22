import init, { leer_archivo } from "../pkg/project_fiuba.js";
import { procesarArchivo } from "./fileProcessor.js";

async function run() {
    await init();

    const fileInput = document.getElementById("file");
    const btnLimpiar = document.getElementById("btn-limpiar");
    const out = document.getElementById("out");

   
    fileInput.addEventListener("change", async () => {
        if (!fileInput.files.length) return;

        out.textContent = "Procesando...";

        const file = fileInput.files[0];
        const buffer = await file.arrayBuffer();

        let bytes = new Uint8Array(buffer);

        try {
            // Descompresión (.Z, .gz) si aplica
            bytes = procesarArchivo(bytes);

          
            const result = leer_archivo(bytes);
            out.textContent = result;
        } catch (error) {
            out.textContent = "Error: " + error.message;
            console.error(error);
        }
    });

   
    btnLimpiar.addEventListener("click", () => {
        fileInput.value = ""; 
        out.textContent = "Esperando archivo...";
        console.log("Pantalla limpia");
    });
}

run();