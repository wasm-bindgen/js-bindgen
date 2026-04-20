export async function importJsBindgen() {
    try {
        const { JsBindgen } = await import("../imports.mjs");
        return JsBindgen;
    }
    catch (error) {
        return error;
    }
}
