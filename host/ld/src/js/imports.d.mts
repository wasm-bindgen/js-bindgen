export declare class JsBindgen {
    #private;
    constructor(module: WebAssembly.Module);
    get importObject(): WebAssembly.Imports;
    extendImportObject(imports: WebAssembly.Imports): void;
    setInstance(instance: WebAssembly.Instance): void;
    instantiate(): Promise<WebAssembly.Instance>;
    static instantiateStreaming(): Promise<WebAssembly.Instance>;
    static instantiateStreaming(...args: Parameters<typeof fetch>): Promise<WebAssembly.Instance>;
}
