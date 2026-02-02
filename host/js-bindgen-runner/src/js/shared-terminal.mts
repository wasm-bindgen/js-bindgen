import { Color, type StyledText } from "./shared.mts"

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
