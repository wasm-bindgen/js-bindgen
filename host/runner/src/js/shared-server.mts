import { Color, type StyledText } from "./shared.mts"

const output = document.getElementById("output") as HTMLPreElement

export function toOutput(text: StyledText[]) {
	function createColor(color: string, text: string) {
		const span = document.createElement("span")
		span.style.color = color
		span.textContent = text
		output.appendChild(span)
	}

	const isAtBottom = Math.abs(output.scrollHeight - output.clientHeight - output.scrollTop) <= 1

	for (const part of text) {
		switch (part.color) {
			case Color.Default:
				const node = document.createTextNode(part.text)
				output.appendChild(node)
				break
			case Color.Green:
				createColor("green", part.text)
				break
			case Color.Yellow:
				createColor("yellow", part.text)
				break
			case Color.Red:
				createColor("red", part.text)
				break
		}
	}

	if (isAtBottom) {
		output.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" })
	}
}
