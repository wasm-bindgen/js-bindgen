declare const JBG_PLACEHOLDER_MEMORY: WebAssembly.Memory
declare const JBG_PLACEHOLDER_JS_EMBED: Record<string, Record<string, any>>
declare const JBG_PLACEHOLDER_IMPORT_OBJECT: WebAssembly.Imports

export class JsBindgen {
	#finished = false
	#importObject: WebAssembly.Imports
	// @ts-expect-error TS6133
	#jsEmbed: Record<string, Record<string, any>>
	// @ts-expect-error TS6133
	#memory = JBG_PLACEHOLDER_MEMORY
	#module: WebAssembly.Module

	constructor(module: WebAssembly.Module) {
		this.#module = module
		this.#jsEmbed = JBG_PLACEHOLDER_JS_EMBED
		this.#importObject = JBG_PLACEHOLDER_IMPORT_OBJECT
	}

	get importObject(): WebAssembly.Imports {
		if (this.#finished) {
			throw "create a new `JsBindgen` class"
		} else {
			return this.#importObject
		}
	}

	extendImportObject(imports: WebAssembly.Imports) {
		if (this.#finished) {
			throw "create a new `JsBindgen` class"
		}

		for (const namespace in imports) {
			if (!this.#importObject[namespace]) {
				continue
			}

			for (const symbol in imports[namespace]) {
				if (this.#importObject[namespace][symbol]) {
					throw `found conflicting symbol: \`${namespace}:${symbol}\``
				}
			}
		}

		for (const namespace in imports) {
			if (!this.#importObject[namespace]) {
				this.#importObject[namespace] = {}
			}

			Object.assign(this.#importObject[namespace], imports[namespace])
		}
	}

	instantiate(): Promise<WebAssembly.Instance> {
		if (this.#finished) {
			throw "create a new `JsBindgen` class"
		}

		return WebAssembly.instantiate(this.#module, this.#importObject).then(instance => {
			this.#finished = true
			return instance
		})
	}

	static instantiateStreaming(): Promise<WebAssembly.Instance>
	static instantiateStreaming(...args: Parameters<typeof fetch>): Promise<WebAssembly.Instance>

	static async instantiateStreaming(
		...args: Parameters<typeof fetch> | []
	): Promise<WebAssembly.Instance> {
		let response

		if (args.length === 0) {
			const url = import.meta.url.replace(/\.mjs$/, ".wasm")
			response = fetch(url)
		} else {
			response = fetch(...args)
		}

		const module = await WebAssembly.compileStreaming(response)

		return new JsBindgen(module).instantiate()
	}
}
