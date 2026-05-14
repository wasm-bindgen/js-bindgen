export class JsBindgen {
    #finished = false;
    #importObject;
    // @ts-expect-error: Used in placeholder.
    // eslint-disable-next-line no-unused-private-class-members, @typescript-eslint/no-explicit-any
    #jsEmbed;
    // @ts-expect-error: Used in placeholder.
    // eslint-disable-next-line no-unused-private-class-members
    #memory = new WebAssembly.Memory({ initial: 17, maximum: 65536 });
    #module;
    constructor(module) {
        this.#module = module;
        this.#jsEmbed = {
			js_sys: {
				'extern_ref': (refPtr) => {
					const [ptr, len] = this.#jsEmbed.js_sys['view.getUint32'](refPtr, 2)
					return { ptr, len }
				},
				'view.getInt32': (ptr, count) => {

					if (this.#jsEmbed.js_sys.isLittleEndian) {
						const view = new Int32Array(this.#memory.buffer, ptr, count)
						return Array.from(view)
					} else {
						const out = new Array(count)
						const view = new DataView(this.#memory.buffer)
						for (let index = 0; index < count; index++) {
							out[index] = view.getInt32(ptr + index * 4, true)
						}
						return out
					}
				},
				'string.decode': (ptr, len) => {
					const decoder = new TextDecoder('utf-8', {
						fatal: false,
						ignoreBOM: false,
					})
					const view = new Uint8Array(this.#memory.buffer, ptr, len)
					return decoder.decode(view)
				},
				'array.js_value.decode': (ptr, len) => {
					const array = new Array(len)
					for (let arrayIndex = 0; arrayIndex < len; arrayIndex++) {
						const [refIndex] = this.#jsEmbed.js_sys['view.getInt32'](ptr + arrayIndex * 4, 1)
						array[arrayIndex] = this.#jsEmbed.js_sys['externref.table'].get(refIndex)
					}
					return array
				},
				'array.rust.js_value': (dataPtr) => {
					const { ptr, len } = this.#jsEmbed.js_sys['extern_ref'](dataPtr)
					return this.#jsEmbed.js_sys['array.js_value.decode'](ptr, len)
				},
				'isLittleEndian': (() => {
					const buffer = new ArrayBuffer(2)
					new DataView(buffer).setInt16(0, 256, true)
					return new Int16Array(buffer)[0] === 256;
				})(),
				'view.getUint32': (ptr, count) => {

					if (this.#jsEmbed.js_sys.isLittleEndian) {
						const view = new Uint32Array(this.#memory.buffer, ptr, count)
						return Array.from(view)
					} else {
						const out = new Array(count)
						const view = new DataView(this.#memory.buffer)
						for (let index = 0; index < count; index++) {
							out[index] = view.getUint32(ptr + index * 4, true)
						}
						return out
					}
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
				'console.log': (data) => {
					data = this.#jsEmbed.js_sys['array.rust.js_value'](data)
				globalThis.console.log(data)
				},
				'console.log0': globalThis.console.log,
			},
		};
    }
    get importObject() {
        if (this.#finished) {
            throw new Error("create a new `JsBindgen` class");
        }
        else {
            return this.#importObject;
        }
    }
    extendImportObject(imports) {
        if (this.#finished) {
            throw new Error("create a new `JsBindgen` class");
        }
        for (const namespace in imports) {
            if (!this.#importObject[namespace]) {
                continue;
            }
            for (const symbol in imports[namespace]) {
                if (this.#importObject[namespace][symbol]) {
                    throw new Error(`found conflicting symbol: \`${namespace}:${symbol}\``);
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
            throw new Error("create a new `JsBindgen` class");
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
