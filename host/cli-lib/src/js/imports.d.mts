export type JsBindgenInstance = {
    instance: WebAssembly.Instance;
    exports: WebAssembly.Instance["exports"];
};
export declare class JsBindgen {
    #private;
    constructor(module: WebAssembly.Module, memory?: WebAssembly.Memory);
    get importObject(): WebAssembly.Imports;
    extendImportObject(imports: WebAssembly.Imports): void;
    instantiate(): Promise<JsBindgenInstance>;
    static instantiateStreaming(...args: Parameters<typeof fetch> | []): Promise<JsBindgenInstance>;
}
