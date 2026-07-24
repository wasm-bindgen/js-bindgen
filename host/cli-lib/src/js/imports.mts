declare const JBG_PLACEHOLDER_MEMORY: WebAssembly.Memory
// eslint-disable-next-line @typescript-eslint/no-explicit-any
declare const JBG_PLACEHOLDER_JS_EMBED: Record<string, Record<string, any>>
declare const JBG_PLACEHOLDER_IMPORT_OBJECT: WebAssembly.Imports
declare const JBG_PLACEHOLDER_JS_EXPORT: WebAssembly.Instance["exports"]

export type JsBindgenInstance = {
	instance: WebAssembly.Instance
	exports: WebAssembly.Instance["exports"]
}

export class JsBindgen {
	#finished = false
	// @ts-expect-error: Used by generated imports that catch exceptions.
	// eslint-disable-next-line no-unused-private-class-members
	#instance: WebAssembly.Instance
	#importObject: WebAssembly.Imports
	// @ts-expect-error: Used in placeholder.
	// eslint-disable-next-line no-unused-private-class-members, @typescript-eslint/no-explicit-any
	#jsEmbed: Record<string, Record<string, any>>
	// @ts-expect-error: Used in placeholder.
	// eslint-disable-next-line no-unused-private-class-members
	#memory: WebAssembly.Memory
	#module: WebAssembly.Module

	constructor(module: WebAssembly.Module, memory?: WebAssembly.Memory) {
		this.#module = module

		if (memory) {
			this.#memory = memory
		} else {
			this.#memory = JBG_PLACEHOLDER_MEMORY
		}

		this.#jsEmbed = JBG_PLACEHOLDER_JS_EMBED
		this.#importObject = JBG_PLACEHOLDER_IMPORT_OBJECT
	}

	get importObject(): WebAssembly.Imports {
		if (this.#finished) {
			throw new Error("create a new `JsBindgen` class")
		} else {
			return this.#importObject
		}
	}

	extendImportObject(imports: WebAssembly.Imports) {
		if (this.#finished) {
			throw new Error("create a new `JsBindgen` class")
		}

		for (const namespace in imports) {
			if (!this.#importObject[namespace]) {
				continue
			}

			for (const symbol in imports[namespace]) {
				if (this.#importObject[namespace][symbol]) {
					throw new Error(`found conflicting symbol: \`${namespace}:${symbol}\``)
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

	async instantiate(): Promise<JsBindgenInstance> {
		if (this.#finished) {
			throw new Error("create a new `JsBindgen` class")
		}

		return WebAssembly.instantiate(this.#module, this.#importObject).then(instance => {
			this.#instance = instance
			this.#finished = true

			const jsExports = JBG_PLACEHOLDER_JS_EXPORT
			const exports = Object.assign(
				Object.create(null) as WebAssembly.Instance["exports"],
				instance.exports,
				jsExports
			)
			return {
				instance,
				exports,
			}
		})
	}

	static async instantiateStreaming(
		...args: Parameters<typeof fetch> | []
	): Promise<JsBindgenInstance> {
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
