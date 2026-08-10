import { Color, type StyledText } from "./shared.mts"

export function keepAlive<T>(promise: Promise<T>): Promise<T> {
	// A pending `Atomics.waitAsync` does not keep a command-line event loop
	// alive: https://github.com/denoland/deno/issues/15358. Keep one timer active
	// until the complete test or binary run settles.
	const timer = globalThis.setInterval(() => undefined, 0x7fffffff)
	return promise.finally(() => globalThis.clearInterval(timer))
}

export function colorText(text: StyledText[]): string {
	const green = "\u001b[32m"
	const yellow = "\u001b[33m"
	const red = "\u001b[31m"
	const reset = "\u001b[0m"

	let output = ""

	for (const part of text) {
		switch (part.color) {
			case Color.Default:
				output += part.text
				break
			case Color.Green:
				output += `${green}${part.text}${reset}`
				break
			case Color.Yellow:
				output += `${yellow}${part.text}${reset}`
				break
			case Color.Red:
				output += `${red}${part.text}${reset}`
				break
		}
	}

	return output
}
