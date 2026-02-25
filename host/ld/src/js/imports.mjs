export class JsBindgen {
    #finished = false;
    #importObject;
    // @ts-expect-error TS6133
    #jsEmbed;
    // @ts-expect-error TS6133
    #memory = JBG_PLACEHOLDER_MEMORY;
    #module;
    constructor(module) {
        this.#module = module;
        this.#jsEmbed = JBG_PLACEHOLDER_JS_EMBED;
        this.#importObject = JBG_PLACEHOLDER_IMPORT_OBJECT;
    }
    get importObject() {
        if (this.#finished) {
            throw "create a new `JsBindgen` class";
        }
        else {
            return this.#importObject;
        }
    }
    extendImportObject(imports) {
        if (this.#finished) {
            throw "create a new `JsBindgen` class";
        }
        for (const namespace in imports) {
            if (!this.#importObject[namespace]) {
                continue;
            }
            for (const symbol in imports[namespace]) {
                if (this.#importObject[namespace][symbol]) {
                    throw `found conflicting symbol: \`${namespace}:${symbol}\``;
                }
            }
        }
        for (const namespace in imports) {
            if (!this.#importObject[namespace]) {
                this.#importObject[namespace] = {};
            }
            Object.assign(this.#importObject[namespace], imports[namespace]);
        }
    }
    instantiate() {
        if (this.#finished) {
            throw "create a new `JsBindgen` class";
        }
        return WebAssembly.instantiate(this.#module, this.#importObject).then(instance => {
            this.#finished = true;
            return instance;
        });
    }
    static async instantiateStreaming(...args) {
        let response;
        if (args.length === 0) {
            const url = import.meta.url.replace(/\.mjs$/, ".wasm");
            response = fetch(url);
        }
        else {
            response = fetch(...args);
        }
        const module = await WebAssembly.compileStreaming(response);
        return new JsBindgen(module).instantiate();
    }
}
