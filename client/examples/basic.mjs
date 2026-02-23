export class JsBindgen {
    #finished = false;
    #importObject;
    // @ts-expect-error TS6133
    #instance;
    // @ts-expect-error TS6133
    #jsEmbed;
    // @ts-expect-error TS6133
    #memory = new WebAssembly.Memory({ initial: 17 });
    #module;
    constructor(module) {
        this.#module = module;
        this.#jsEmbed = {
			js_sys: {
				'string.decode': (ptr, len) => {
					const decoder = new TextDecoder('utf-8', {
						fatal: false,
						ignoreBOM: false,
					})
					const view = new Uint8Array(this.#memory.buffer, ptr, len)
				
					return decoder.decode(view)
				},
				'externref.table': (() => {
					const table = new WebAssembly.Table({ initial: 2, element: 'externref' })
					table.set(1, null)
					return table
				})(),
			},
		};
        this.#importObject = {
			js_bindgen: { memory: this.#memory },
			js_sys: {
				'externref.table': this.#jsEmbed.js_sys['externref.table'],
				'string_decode': (array, len) => {
					array >>>= 0
					len >>>= 0
					return this.#jsEmbed.js_sys['string.decode'](array, len)
				},
			},
			web_sys: {
				'console.log2': globalThis.console.log,
				'console.log': globalThis.console.log,
				'console.log0': globalThis.console.log,
			},
		};
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
    setInstance(instance) {
        if (this.#finished) {
            throw "create a new `JsBindgen` class";
        }
        this.#instance = instance;
        this.#finished = true;
    }
    instantiate() {
        if (this.#finished) {
            throw "create a new `JsBindgen` class";
        }
        this.#finished = true;
        return WebAssembly.instantiate(this.#module, this.#importObject);
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
